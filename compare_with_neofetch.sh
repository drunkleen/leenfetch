#!/usr/bin/env bash
set -euo pipefail

# Defaults
RUNS=100
WARMUP=3
CMD1="/bin/neofetch"
CMD2="leenfetch"
PREPARE=""            # --prepare hook for hyperfine
AUTO_RERUN=true       # auto rerun once on detected outliers/noise
RERUN_WARMUP_BOOST=5  # extra warmups on rerun

usage() {
  cat <<EOF
Usage: $0 [-r RUNS] [-w WARMUP] [--prepare 'CMD'] [--auto-rerun|--no-auto-rerun] [CMD1] [CMD2]

Defaults:
  RUNS=$RUNS
  WARMUP=$WARMUP
  CMD1="$CMD1"
  CMD2="$CMD2"

Examples:
  $0 -r 200 -w 5 --prepare 'sleep 0.1' "./target/release/leenfetch" "leenfetch"
  $0 --no-auto-rerun
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
    -h|--help) usage; exit 0 ;;
    --) shift; break ;;
    -*)
      echo "Invalid option: $1" >&2; usage; exit 2 ;;
    *)
      # positional
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

echo "▶ Running hyperfine with $RUNS runs (warmup $WARMUP) for:"
echo "   1) $CMD1"
echo "   2) $CMD2"
[[ -n "$PREPARE" ]] && echo "   prepare: $PREPARE"
echo

run_hyperfine() {
  local runs="$1" warm="$2" out="$3" prepare="$4"
  local args=( --warmup "$warm" --runs "$runs" --export-json "$out" --style basic )
  [[ -n "$prepare" ]] && args+=( --prepare "$prepare" )
  hyperfine "${args[@]}" "$CMD1" "$CMD2"
}

# --- 1st pass
run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"

# Extract metrics
name1=$(jq -r '.results[0].command' "$RESULTS_JSON")
name2=$(jq -r '.results[1].command' "$RESULTS_JSON")
mean1=$(jq -r '.results[0].mean'     "$RESULTS_JSON")
mean2=$(jq -r '.results[1].mean'     "$RESULTS_JSON")
med1=$(jq -r  '.results[0].median'   "$RESULTS_JSON")
med2=$(jq -r  '.results[1].median'   "$RESULTS_JSON")
std1=$(jq -r  '.results[0].stddev'   "$RESULTS_JSON")
std2=$(jq -r  '.results[1].stddev'   "$RESULTS_JSON")
min1=$(jq -r  '.results[0].min'      "$RESULTS_JSON")
min2=$(jq -r  '.results[1].min'      "$RESULTS_JSON")
max1=$(jq -r  '.results[0].max'      "$RESULTS_JSON")
max2=$(jq -r  '.results[1].max'      "$RESULTS_JSON")

# Heuristic "outlier/noise" detection (cheap but effective):
# - high relative stddev OR very wide min..max ratio
is_noisy() {
  local mean="$1" std="$2" min="$3" max="$4"
  # avoid divide-by-zero
  awk -v m="$mean" -v s="$std" -v min="$min" -v max="$max" '
    BEGIN {
      if (m <= 0) { print 1; exit }         # nonsense mean -> call noisy
      rel = s / m;
      spread = (min > 0 ? max/min : 9999);
      # thresholds: rel σ > 0.5 (50%) OR range spread > 4x
      if (rel > 0.50 || spread > 4.0) print 1; else print 0;
    }'
}

noisy1=$(is_noisy "$mean1" "$std1" "$min1" "$max1")
noisy2=$(is_noisy "$mean2" "$std2" "$min2" "$max2")
reran=false

if $AUTO_RERUN && { [[ "$noisy1" -eq 1 ]] || [[ "$noisy2" -eq 1 ]]; }; then
  echo "⚠️  Statistical outliers/noise detected. Re-running once with extra warmup..."
  echo "   Tip: keep the system quiet; consider using '--prepare' (e.g., 'sleep 0.1')."
  # increase warmups and re-run once
  WARMUP=$(( WARMUP + RERUN_WARMUP_BOOST ))
  run_hyperfine "$RUNS" "$WARMUP" "$RESULTS_JSON" "$PREPARE"
  reran=true

  # re-extract metrics after rerun
  name1=$(jq -r '.results[0].command' "$RESULTS_JSON")
  name2=$(jq -r '.results[1].command' "$RESULTS_JSON")
  mean1=$(jq -r '.results[0].mean'     "$RESULTS_JSON")
  mean2=$(jq -r '.results[1].mean'     "$RESULTS_JSON")
  med1=$(jq -r  '.results[0].median'   "$RESULTS_JSON")
  med2=$(jq -r  '.results[1].median'   "$RESULTS_JSON")
  std1=$(jq -r  '.results[0].stddev'   "$RESULTS_JSON")
  std2=$(jq -r  '.results[1].stddev'   "$RESULTS_JSON")
  min1=$(jq -r  '.results[0].min'      "$RESULTS_JSON")
  min2=$(jq -r  '.results[1].min'      "$RESULTS_JSON")
  max1=$(jq -r  '.results[0].max'      "$RESULTS_JSON")
  max2=$(jq -r  '.results[1].max'      "$RESULTS_JSON")
fi

# Pretty print summary
printf "\n=== Summary after %d runs (warmup %d)%s ===\n" "$RUNS" "$WARMUP" "$($reran && echo ' [rerun]')"
printf "%-30s %12s %12s %12s %12s %12s\n" "Command" "Mean(s)" "Median(s)" "StdDev(s)" "Min(s)" "Max(s)"
printf "%-30s %12.6f %12.6f %12.6f %12.6f %12.6f\n" "$name1" "$mean1" "$med1" "$std1" "$min1" "$max1"
printf "%-30s %12.6f %12.6f %12.6f %12.6f %12.6f\n" "$name2" "$mean2" "$med2" "$std2" "$min2" "$max2"

# Decide winner (mean first, then median)
winner=""
loser=""
w_mean=0
l_mean=0

cmp=$(awk -v a="$mean1" -v b="$mean2" 'BEGIN{ if (a<b) print -1; else if (a>b) print 1; else print 0 }')
if [[ "$cmp" -lt 0 ]]; then
  winner="$name1"; loser="$name2"; w_mean="$mean1"; l_mean="$mean2"
elif [[ "$cmp" -gt 0 ]]; then
  winner="$name2"; loser="$name1"; w_mean="$mean2"; l_mean="$mean1"
else
  # tie by mean -> compare median
  cmpm=$(awk -v a="$med1" -v b="$med2" 'BEGIN{ if (a<b) print -1; else if (a>b) print 1; else print 0 }')
  if [[ "$cmpm" -lt 0 ]]; then
    winner="$name1"; loser="$name2"; w_mean="$mean1"; l_mean="$mean2"
  elif [[ "$cmpm" -gt 0 ]]; then
    winner="$name2"; loser="$name1"; w_mean="$mean2"; l_mean="$mean1"
  else
    echo -e "\n⚖️  Result: Statistical tie on mean and median."
    exit 0
  fi
fi

# Percent advantage
pct=$(awk -v w="$w_mean" -v l="$l_mean" 'BEGIN{ printf "%.2f", (1 - w/l) * 100 }')

echo
echo "✅ Faster overall (by mean, tiebreaker median):"
echo "Winner: $winner"
printf "   Advantage: %s%% faster (%.6fs vs %.6fs mean)\n" "$pct" "$w_mean" "$l_mean"

# If still noisy, surface the hint like hyperfine does
post_noisy1=$(is_noisy "$mean1" "$std1" "$min1" "$max1")
post_noisy2=$(is_noisy "$mean2" "$std2" "$min2" "$max2")
if [[ "$post_noisy1" -eq 1 || "$post_noisy2" -eq 1 ]]; then
  cat <<'NOTE'

⚠️  Statistical outliers were detected.
   Re-run on a quiet system and consider using:
     --prepare 'sleep 0.1'      # simple stabilizer
     --warmup N                  # higher warmup
   You can pass --prepare to this script; it forwards to hyperfine.

NOTE
fi
