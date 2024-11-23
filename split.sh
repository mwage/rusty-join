#!/bin/bash

# Input CSV file
input_file="$1"

# Check if the input file exists
if [ ! -f "$input_file" ]; then
  echo "File does not exist!"
  exit 1
fi

# Percentage for the first file (passed as second argument)
percentage="$2"

# Check if the percentage argument is provided and valid
if [ -z "$percentage" ] || ! [[ "$percentage" =~ ^[0-9]+$ ]] || [ "$percentage" -lt 1 ] || [ "$percentage" -gt 100 ]; then
  echo "Please provide a valid percentage (between 1 and 100) as the second argument."
  exit 1
fi

# Get total number of lines in the file
total_lines=$(wc -l < "$input_file")

# Calculate the number of lines for the first file (percentage of total lines, rounded up)
lines_for_first_file=$(( (total_lines * percentage + 99) / 100 ))

# Get the name of the input file without the extension
base_name=$(basename "$input_file" .csv)

# Output the first 'percentage' of lines to the first file (named as base_name_percentage.csv)
output_file="${base_name}_${percentage}.csv"
head -n "$lines_for_first_file" "$input_file" > "$output_file"

echo "First $percentage% saved in '$output_file'"