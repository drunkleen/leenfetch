#!/usr/bin/env bash
set -euo pipefail

# Defaults (tuned to reduce outliers)
RUNS=100
WARMUP=5
CMD1="/bin/neofetch"
CMD2="./target/release/leenfetch"
# CMD2="leenfetch"
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
          [CMD1] [CMD2]

Defaults:
  RUNS=$RUNS
  WARMUP=$WARMUP
  PREPARE="$PREPARE"
  CMD1="$CMD1"
  CMD2="$CMD2"

Examples:
  $0 -r 200 -w 7 --prepare 'sleep 0.1' "./target/release/leenfetch" "leenfetch"
  $0 --pin-cpu 3 --nice --ionice
  $0 --perf-governor   # needs sudo; will skip safely if not available
EOF
}

# Parse flags
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
    -*)
      echo "Invalid option: $1" >&2; usage; exit 2 ;;
    *)
      break ;;
  esac
done

# Positional overrides for commands
if [[ $# -ge 1 ]]; then CMD1="$1"; fi
if [[ $# -ge 2 ]]; then CMD2="$2"; fi
if [[ $# -gt  2 ]]; then echo "Too many args"; usage; exit 2; fi

# Requirements check
for bin in hyperfine jq awk; do
  command -v "$bin" >/dev/null 2>&1 || { echo "Missing dependency: $bin"; exit 1; }
done

RESULTS_JSON="$(mktemp --suffix=.json)"
cleanup() { rm -f "$RESULTS_JSON"; }
trap cleanup EXIT

# Compose an optional launcher for stabilization (taskset/nice/ionice)
launcher=()
if [[ -n "${PIN_CPU}" ]]; then
  if command -v taskset >/dev/null 2>&1; then
    launcher+=(taskset -c "${PIN_CPU}")
  else
    echo "Note: taskset not found; skipping CPU pinning."
  fi
fi
if $USE_NICE; then
  launcher+=(nice -n 19)
fi
if $USE_IONICE; then
  if command -v ionice >/dev/null 2>&1; then
    launcher+=(ionice -c 3)
  else
    echo "Note: ionice not found; skipping I/O niceness."
  fi
fi

# Optional: performance governor (best-effort; requires sudo)
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

# Wrap commands with launcher if any
wrap_cmd() {
  local cmd="$1"
  if [[ "${#launcher[@]}" -gt 0 ]]; then
    printf "%q " "${launcher[@]}"
    printf "%s" "$cmd"
  else
    printf "%s" "$cmd"
  fi
}

CMD1_WRAPPED="$(wrap_cmd "$CMD1")"
CMD2_WRAPPED="$(wrap_cmd "$CMD2")"

# Minimal header
printf "Running %d runs (warmup %d) → [%s] vs [%s]\n" "$RUNS" "$WARMUP" "$CMD1_WRAPPED" "$CMD2_WRAPPED"
[[ -n "$PREPARE" ]] && printf "prepare: %s\n" "$PREPARE"

run_hyperfine() {
  local runs="$1" warm="$2" out="$3" prepare="$4"
  local args=( --warmup "$warm" --runs "$runs" --export-json "$out" --style full )
  [[ -n "$prepare" ]] && args+=( --prepare "$prepare" )
  # NOTE: avoid --quiet to keep progress bars; send output to TTY for a clean UI
  hyperfine "${args[@]}" "$CMD1_WRAPPED" "$CMD2_WRAPPED" >/dev/tty
}

# --- 1st pass
run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"

extract() { jq -r "$1" "$RESULTS_JSON"; }

name1=$(extract '.results[0].command')
name2=$(extract '.results[1].command')
mean1=$(extract '.results[0].mean');     mean2=$(extract '.results[1].mean')
med1=$(extract  '.results[0].median');   med2=$(extract  '.results[1].median')
std1=$(extract  '.results[0].stddev');   std2=$(extract  '.results[1].stddev')
min1=$(extract  '.results[0].min');      min2=$(extract  '.results[1].min')
max1=$(extract  '.results[0].max');      max2=$(extract  '.results[1].max')

# Heuristic noise detector (rel stddev or wide min..max spread)
is_noisy() {
  local mean="$1" std="$2" min="$3" max="$4"
  awk -v m="$mean" -v s="$std" -v min="$min" -v max="$max" '
    BEGIN {
      if (m <= 0) { print 1; exit }
      rel = s / m
      spread = (min > 0 ? max/min : 9999)
      if (rel > 0.50 || spread > 4.0) print 1; else print 0
    }'
}

noisy1=$(is_noisy "$mean1" "$std1" "$min1" "$max1")
noisy2=$(is_noisy "$mean2" "$std2" "$min2" "$max2")
reran=false

if $AUTO_RERUN && { [[ "$noisy1" -eq 1 ]] || [[ "$noisy2" -eq 1 ]]; }; then
  printf "Noise detected → one rerun with extra warmup (+%d)\n" "$RERUN_WARMUP_BOOST"
  WARMUP=$(( WARMUP + RERUN_WARMUP_BOOST ))
  run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"
  reran=true

  name1=$(extract '.results[0].command')
  name2=$(extract '.results[1].command')
  mean1=$(extract '.results[0].mean');     mean2=$(extract '.results[1].mean')
  med1=$(extract  '.results[0].median');   med2=$(extract  '.results[1].median')
  std1=$(extract  '.results[0].stddev');   std2=$(extract  '.results[1].stddev')
  min1=$(extract  '.results[0].min');      min2=$(extract  '.results[1].min')
  max1=$(extract  '.results[0].max');      max2=$(extract  '.results[1].max')
fi

# Decide winner using mean (tie-breaker median)
winner="" loser="" w_mean=0 l_mean=0
cmp=$(awk -v a="$mean1" -v b="$mean2" 'BEGIN{ if (a<b) print -1; else if (a>b) print 1; else print 0 }')
if [[ "$cmp" -lt 0 ]]; then
  winner="$name1"; loser="$name2"; w_mean="$mean1"; l_mean="$mean2"
elif [[ "$cmp" -gt 0 ]]; then
  winner="$name2"; loser="$name1"; w_mean="$mean2"; l_mean="$mean1"
else
  cmpm=$(awk -v a="$med1" -v b="$med2" 'BEGIN{ if (a<b) print -1; else if (a>b) print 1; else print 0 }')
  if [[ "$cmpm" -lt 0 ]]; then
    winner="$name1"; loser="$name2"; w_mean="$mean1"; l_mean="$mean2"
  elif [[ "$cmpm" -gt 0 ]]; then
    winner="$name2"; loser="$name1"; w_mean="$mean2"; l_mean="$mean1"
  else
    printf "\nResult: statistical tie (mean & median)\n"
    exit 0
  fi
fi

pct=$(awk -v w="$w_mean" -v l="$l_mean" 'BEGIN{ printf "%.2f", (1 - w/l) * 100 }')

# Compact summary
printf "\nSummary (%d runs, warmup %d%s)\n" "$RUNS" "$WARMUP" "$($reran && echo ', rerun')"
printf "Winner: %s\n" "$winner"
printf "Advantage: %s%% faster (mean %.6fs vs %.6fs)\n" "$pct" "$w_mean" "$l_mean"
printf "Medians:  %-.6fs vs %-.6fs | StdDevs: %-.6fs vs %-.6fs\n" "$med1" "$med2" "$std1" "$std2"

post_noisy1=$(is_noisy "$mean1" "$std1" "$min1" "$max1")
post_noisy2=$(is_noisy "$mean2" "$std2" "$min2" "$max2")
if [[ "$post_noisy1" -eq 1 || "$post_noisy2" -eq 1 ]]; then
  printf "\nNote: outliers still present. Try: higher --warmup, --runs, close background tasks, or --pin-cpu.\n"
fi
