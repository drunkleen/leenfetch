#!/usr/bin/env bash
set -euo pipefail

# -------------------- Defaults (tuned to reduce outliers) --------------------
RUNS=100
WARMUP=5

# Commands (can be overridden positionally)
CMD1="fastfetch"
CMD2="neofetch"
CMD3="./target/release/leenfetch"

PREPARE="sleep 0.1"   # small pause between runs; stabilizes CPU scheduler
AUTO_RERUN=true        # re-run once on detected noise
RERUN_WARMUP_BOOST=5   # extra warmups on rerun

# Optional stabilizers (off by default)
PIN_CPU=""             # e.g., "3" to pin on CPU core 3
USE_NICE=false         # run with low priority via `nice`
USE_IONICE=false       # run with idle I/O via `ionice`
USE_PERF_GOV=false     # try to switch CPU governor to performance (requires sudo)

usage() {
  cat <<EOF
Usage: $0 [-r RUNS] [-w WARMUP] [--prepare 'CMD'] [--auto-rerun|--no-auto-rerun]
          [--pin-cpu N] [--nice] [--ionice] [--perf-governor]
          [CMD1] [CMD2] [CMD3]

Defaults:
  RUNS=$RUNS
  WARMUP=$WARMUP
  PREPARE="$PREPARE"
  CMD1="$CMD1"
  CMD2="$CMD2"
  CMD3="$CMD3"

Examples:
  $0 -r 200 -w 7 --prepare 'sleep 0.1' "./target/release/leenfetch" "leenfetch" "fastfetch"
  $0 --pin-cpu 3 --nice --ionice
  $0 --perf-governor
EOF
}

# ------------------------------ Parse flags ---------------------------------
positional=()
while [[ $# -gt 0 ]]; do
  case "${1:-}" in
    -r) RUNS="$2"; shift 2 ;;
    -w) WARMUP="$2"; shift 2 ;;
    --prepare) PREPARE="$2"; shift 2 ;;
    --auto-rerun) AUTO_RERUN=true; shift ;;
    --no-auto-rerun) AUTO_RERUN=false; shift ;;
    --pin-cpu) PIN_CPU="$2"; shift 2 ;;
    --nice) USE_NICE=true; shift ;;
    --ionice) USE_IONICE=true; shift ;;
    --perf-governor) USE_PERF_GOV=true; shift ;;
    -h|--help) usage; exit 0 ;;
    --) shift; break ;;
    -*) echo "Invalid option: $1" >&2; usage; exit 2 ;;
    *) positional+=("$1"); shift ;;
  esac
done
# Remaining args after `--`
if [[ $# -gt 0 ]]; then
  while [[ $# -gt 0 ]]; do positional+=("$1"); shift; done
fi

# Apply positional overrides (up to 3)
case "${#positional[@]}" in
  0) ;;
  1) CMD1="${positional[0]}" ;;
  2) CMD1="${positional[0]}"; CMD2="${positional[1]}" ;;
  3) CMD1="${positional[0]}"; CMD2="${positional[1]}"; CMD3="${positional[2]}" ;;
  *) echo "Too many positional args (max 3)."; usage; exit 2 ;;
esac

# ------------------------------ Requirements --------------------------------
for bin in hyperfine jq awk column; do
  command -v "$bin" >/dev/null 2>&1 || { echo "Missing dependency: $bin"; exit 1; }
done

RESULTS_JSON="$(mktemp --suffix=.json)"
cleanup() { rm -f "$RESULTS_JSON"; }
trap cleanup EXIT

# ---------------------------- Optional launcher -----------------------------
launcher=()
if [[ -n "${PIN_CPU}" ]]; then
  if command -v taskset >/dev/null 2>&1; then
    launcher+=(taskset -c "${PIN_CPU}")
  else
    echo "Note: taskset not found; skipping CPU pinning."
  fi
fi
$USE_NICE && launcher+=(nice -n 19)
if $USE_IONICE; then
  if command -v ionice >/dev/null 2>&1; then
    launcher+=(ionice -c 3)
  else
    echo "Note: ionice not found; skipping I/O niceness."
  fi
fi

if $USE_PERF_GOV; then
  if command -v cpupower >/dev/null 2>&1; then
    if sudo -n true 2>/dev/null; then
      sudo cpupower frequency-set -g performance || echo "Note: failed to set performance governor."
    else
      echo "Note: sudo without password not available; skipping --perf-governor."
    fi
  else
    echo "Note: cpupower not found; skipping --perf-governor."
  fi
fi

wrap_cmd() {
  local cmd="$1"
  if [[ "${#launcher[@]}" -gt 0 ]]; then
    printf "%q " "${launcher[@]}"
    printf "%s" "$cmd"
  else
    printf "%s" "$cmd"
  fi
}

# Build the command list (filter empty)
raw_cmds=("$CMD1" "$CMD2" "$CMD3")
commands=()
for c in "${raw_cmds[@]}"; do
  [[ -n "${c:-}" ]] && commands+=("$(wrap_cmd "$c")")
done

# Deduplicate while preserving order
deduped=()
seen=""
for c in "${commands[@]}"; do
  if [[ ",$seen," != *",$c,"* ]]; then
    deduped+=("$c")
    seen="${seen:+$seen,}$c"
  fi
done
commands=("${deduped[@]}")

# Require 2–3 commands
if (( ${#commands[@]} < 2 || ${#commands[@]} > 3 )); then
  echo "Need between 2 and 3 commands after filtering (got ${#commands[@]})."; exit 2
fi

# ------------------------------ Runner wrapper ------------------------------
run_hyperfine() {
  local runs="$1" warm="$2" out="$3" prepare="$4"
  local args=( --warmup "$warm" --runs "$runs" --export-json "$out" --style full )
  [[ -n "$prepare" ]] && args+=( --prepare "$prepare" )
  hyperfine "${args[@]}" "${commands[@]}" >/dev/tty
}

printf "Running %d runs (warmup %d) →\n" "$RUNS" "$WARMUP"
for c in "${commands[@]}"; do printf "  - %s\n" "$c"; done
[[ -n "$PREPARE" ]] && printf "prepare: %s\n" "$PREPARE"

# ----------------------------- First measurement ----------------------------
run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"

# Heuristic noise detection across all contenders
any_noisy() {
  # Returns success (0) if any contender looks noisy
  jq -e --argjson thr_rel 0.50 --argjson thr_spread 4.0 '
    [ .results[]
      | ( ( .mean?    // 0 ) as $m
        | ( .stddev?  // $m ) as $s
        | ( .min?     // 0 ) as $min
        | ( .max?     // $min ) as $max
        | ($m == 0)
          or ($s/$m > $thr_rel)
          or ( ($min > 0) and ($max/$min > $thr_spread) )
        )
    ] | any
  ' "$RESULTS_JSON" >/dev/null
}

reran=false
if $AUTO_RERUN && any_noisy; then
  printf "Noise detected → one rerun with extra warmup (+%d)\n" "$RERUN_WARMUP_BOOST"
  WARMUP=$(( WARMUP + RERUN_WARMUP_BOOST ))
  run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"
  reran=true
fi

# ------------------------------- Summarization ------------------------------
# Sorted table by mean ascending
table="$(jq -r '
  [ .results[] | {cmd:.command, mean, median, stddev, min, max} ]
  | sort_by(.mean)
  | (["Rank","Command","Mean(s)","Median(s)","StdDev(s)","Min(s)","Max(s)"]),
    ( to_entries[]
      | [ (.+1|tostring),
          .value.cmd,
          (.value.mean    // "NaN"),
          (.value.median  // "NaN"),
          (.value.stddev  // "NaN"),
          (.value.min     // "NaN"),
          (.value.max     // "NaN") ] )
  | @tsv
' "$RESULTS_JSON" | column -t -s $'\t')"

echo
echo "Results (sorted by mean):"
echo "$table"

# Winner, runner-up, slowest
winner_cmd="$(jq -r '.results | sort_by(.mean) | .[0].command' "$RESULTS_JSON")"
runner_cmd="$(jq -r '.results | sort_by(.mean) | .[1].command' "$RESULTS_JSON")"
slow_cmd="$(jq -r '.results | sort_by(.mean) | .[-1].command' "$RESULTS_JSON")"
w_mean="$(jq -r '.results | sort_by(.mean) | .[0].mean' "$RESULTS_JSON")"
r_mean="$(jq -r '.results | sort_by(.mean) | .[1].mean' "$RESULTS_JSON")"
s_mean="$(jq -r '.results | sort_by(.mean) | .[-1].mean' "$RESULTS_JSON")"

adv_vs_runner="$(awk -v w="$w_mean" -v r="$r_mean" 'BEGIN{ if (w>0&&r>0) printf "%.2f", (1 - w/r)*100; else print "NaN" }')"
adv_vs_slowest="$(awk -v w="$w_mean" -v s="$s_mean" 'BEGIN{ if (w>0&&s>0) printf "%.2f", (1 - w/s)*100; else print "NaN" }')"

printf "\nSummary (%d runs, warmup %d%s)\n" "$RUNS" "$WARMUP" "$($reran && echo ', rerun')"
printf "Winner: %s\n" "$winner_cmd"
printf "Advantage vs #2: %s%% faster (%.6fs vs %.6fs)\n" "$adv_vs_runner" "$w_mean" "$r_mean"
printf "Advantage vs slowest: %s%% faster (%.6fs vs %.6fs)\n" "$adv_vs_slowest" "$w_mean" "$s_mean"

# Post-run noise hint (if still noisy)
if any_noisy; then
  printf "\nNote: outliers still present. Try: higher --warmup/--runs, close background tasks, --pin-cpu, or disable turbo.\n"
fi
