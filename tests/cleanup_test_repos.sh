#!/bin/bash
set -e

echo "Cleaning up test repositories..."
cd "$(dirname "$0")"

# Find all test directories
TEST_DIRS=$(find . -type d -name "????-*" | sort)

for test_dir in $TEST_DIRS; do
    echo "Cleaning up: $test_dir"
    
    # Remove git repositories and remotes
    rm -rf "$test_dir/.git" "$test_dir/remote"
    
    # Remove any untracked files created during tests
    if [ -d "$test_dir/untracked_dir" ]; then
        rm -rf "$test_dir/untracked_dir"
    fi
    
    find "$test_dir" -name "untracked*" -type f -delete
done

echo "✅ All test repositories cleaned up successfully!"