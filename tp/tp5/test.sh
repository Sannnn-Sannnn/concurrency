#!/bin/bash

# ============================================
# CONFIG
# ============================================

QUEUE_TYPES=("blocking" "nonblocking")

TESTS=(
  "4 4 100"
  "4 4 100000"
  "4 4 1000000"

  "1 1 100000"
  "2 2 100000"

  "4 1 100000"

  "8 8 100000"
  "8 2 100000"
  "16 2 100000"

  "2 8 100000"
  "2 16 100000"

  "50 20 1000000"
)

# ============================================
# RUN
# ============================================

echo "Building release binary..."
cargo build --release

echo ""
echo "========================================"
echo "RUNNING BENCHMARKS"
echo "========================================"

for queue in "${QUEUE_TYPES[@]}"
do
  echo ""
  echo "########################################"
  echo "QUEUE TYPE: $queue"
  echo "########################################"

  for test in "${TESTS[@]}"
  do
    read producers consumers items <<< "$test"

    echo ""
    echo "----------------------------------------"
    echo "P=$producers C=$consumers ITEMS=$items"
    echo "----------------------------------------"

    ./target/release/tp5 \
      --queue "$queue" \
      --producers "$producers" \
      --consumers "$consumers" \
      --items "$items"
  done
done
