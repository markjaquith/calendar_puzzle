#!/usr/bin/env bash

# Define valid month-day combinations
declare -A days_in_month=(
  [January]=31 [February]=28 [March]=31 [April]=30
  [May]=31 [June]=30 [July]=31 [August]=31
  [September]=30 [October]=31 [November]=30 [December]=31
)

# Define the weekdays
weekdays=(Monday Tuesday Wednesday Thursday Friday Saturday Sunday)

# Loop through each combination
for month in "${!days_in_month[@]}"; do
  for day in $(seq 1 "${days_in_month[$month]}"); do
    # Create the directory for this month/day
    mkdir -p "solutions/${month}/${day}"
    for weekday in "${weekdays[@]}"; do
      # Run the command and redirect to the specified file
      ./target/release/calendar_puzzle --raw --all \
        --month "$month" \
        --day "$day" \
        --weekday "$weekday" \
        2>/dev/null \
        > "solutions/${month}/${day}/${weekday}.txt"
    done
  done
done
