#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

MIN_PERFORMANCE=70
MIN_ACCESSIBILITY=80
MIN_BEST_PRACTICES=80
MIN_SEO=80
MAX_LCP=3000
MAX_FCP=2000
MAX_CLS=0.25
MAX_TBT=500

DIST_DIR="dist"
PORT=8080
RESULTS_DIR=".benchmark-results"

command -v lighthouse &> /dev/null || npm install -g lighthouse

mkdir -p "$RESULTS_DIR"

npx serve -s "$DIST_DIR" -l "$PORT" &
SERVER_PID=$!
sleep 3
trap "kill $SERVER_PID 2>/dev/null || true" EXIT

lighthouse "http://localhost:$PORT" \
    --output=json \
    --output-path="$RESULTS_DIR/report.json" \
    --chrome-flags="--headless --no-sandbox --disable-gpu" \
    --preset=desktop \
    --quiet

PERF=$(jq '.categories.performance.score * 100 | floor' "$RESULTS_DIR/report.json")
A11Y=$(jq '.categories.accessibility.score * 100 | floor' "$RESULTS_DIR/report.json")
BP=$(jq '.categories["best-practices"].score * 100 | floor' "$RESULTS_DIR/report.json")
SEO=$(jq '.categories.seo.score * 100 | floor' "$RESULTS_DIR/report.json")
LCP=$(jq '.audits["largest-contentful-paint"].numericValue | floor' "$RESULTS_DIR/report.json")
FCP=$(jq '.audits["first-contentful-paint"].numericValue | floor' "$RESULTS_DIR/report.json")
CLS=$(jq '.audits["cumulative-layout-shift"].numericValue' "$RESULTS_DIR/report.json")
TBT=$(jq '.audits["total-blocking-time"].numericValue | floor' "$RESULTS_DIR/report.json")

LCP_DISPLAY=$(jq -r '.audits["largest-contentful-paint"].displayValue' "$RESULTS_DIR/report.json")
FCP_DISPLAY=$(jq -r '.audits["first-contentful-paint"].displayValue' "$RESULTS_DIR/report.json")
CLS_DISPLAY=$(jq -r '.audits["cumulative-layout-shift"].displayValue' "$RESULTS_DIR/report.json")
TBT_DISPLAY=$(jq -r '.audits["total-blocking-time"].displayValue' "$RESULTS_DIR/report.json")

FAILED=0

echo ""
echo "Scores:"
[ "$PERF" -ge "$MIN_PERFORMANCE" ] && echo -e "  Performance:    ${GREEN}$PERF${NC}" || { echo -e "  Performance:    ${RED}$PERF${NC} (min: $MIN_PERFORMANCE)"; FAILED=1; }
[ "$A11Y" -ge "$MIN_ACCESSIBILITY" ] && echo -e "  Accessibility:  ${GREEN}$A11Y${NC}" || { echo -e "  Accessibility:  ${RED}$A11Y${NC} (min: $MIN_ACCESSIBILITY)"; FAILED=1; }
[ "$BP" -ge "$MIN_BEST_PRACTICES" ] && echo -e "  Best Practices: ${GREEN}$BP${NC}" || { echo -e "  Best Practices: ${RED}$BP${NC} (min: $MIN_BEST_PRACTICES)"; FAILED=1; }
[ "$SEO" -ge "$MIN_SEO" ] && echo -e "  SEO:            ${GREEN}$SEO${NC}" || { echo -e "  SEO:            ${RED}$SEO${NC} (min: $MIN_SEO)"; FAILED=1; }

echo ""
echo "Core Web Vitals:"
[ "$LCP" -le "$MAX_LCP" ] && echo -e "  LCP: ${GREEN}$LCP_DISPLAY${NC}" || { echo -e "  LCP: ${RED}$LCP_DISPLAY${NC} (max: ${MAX_LCP}ms)"; FAILED=1; }
[ "$FCP" -le "$MAX_FCP" ] && echo -e "  FCP: ${GREEN}$FCP_DISPLAY${NC}" || { echo -e "  FCP: ${RED}$FCP_DISPLAY${NC} (max: ${MAX_FCP}ms)"; FAILED=1; }

CLS_INT=$(echo "$CLS * 1000" | bc | cut -d'.' -f1)
MAX_CLS_INT=$(echo "$MAX_CLS * 1000" | bc | cut -d'.' -f1)
[ "$CLS_INT" -le "$MAX_CLS_INT" ] && echo -e "  CLS: ${GREEN}$CLS_DISPLAY${NC}" || { echo -e "  CLS: ${RED}$CLS_DISPLAY${NC} (max: $MAX_CLS)"; FAILED=1; }
[ "$TBT" -le "$MAX_TBT" ] && echo -e "  TBT: ${GREEN}$TBT_DISPLAY${NC}" || { echo -e "  TBT: ${RED}$TBT_DISPLAY${NC} (max: ${MAX_TBT}ms)"; FAILED=1; }

echo ""
if [ "$FAILED" -eq 1 ]; then
    echo -e "${RED}BENCHMARK FAILED${NC}"
    echo -e "${RED}FIX BENCHES BEFORE DEPLOYING${NC}"
    exit 1
else
    echo -e "${GREEN}BENCHMARK PASSED${NC}"
    exit 0
fi
