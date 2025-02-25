#!/bin/bash
set -e

echo "Setting up test repositories..."
cd "$(dirname "$0")"

# Find all test directories
TEST_DIRS=$(find . -type d -name "????-*" | sort)

for test_dir in $TEST_DIRS; do
    echo ""
    echo "Setting up repository for: $test_dir"
    
    # Remove any existing git repos and remotes
    rm -rf "$test_dir/.git" "$test_dir/remote"
    
    # Initialize git repo
    cd "$test_dir"
    git init
    
    # Create remote repository
    mkdir -p remote
    cd remote
    git init --bare
    cd ..
    
    # Add remote
    git remote add origin ./remote
    
    # Initial commit
    git add .
    git commit -m "Initial commit"
    
    # Push to remote
    git push -u origin main
    
    # Return to parent directory
    cd ..
done

echo ""
echo "✅ All test repositories set up successfully!"
echo "Run the tests with: ./run_all_tests.sh"