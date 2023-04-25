#!/bin/bash

# Get the list of changed paths in the latest commit
changed_paths=$(git diff-tree --no-commit-id --name-only -r HEAD)

# Variables to track changes in specified paths
source_changed=false
website_changed=false

# Check if any of the specified paths have changed
for path in $changed_paths; do
  if [[ $path == Cargo.* || $path == src/* || $path == .build/* ]]; then
    source_changed=true
  elif [[ $path == website/* ]]; then
    website_changed=true
  fi
done

# Determine the result based on the changes detected
if $source_changed && $website_changed; then
  result="all"
elif $source_changed; then
  result="source"
elif $website_changed; then
  result="website"
else
  result="none"
fi

echo "Result: $result"
exit 0
