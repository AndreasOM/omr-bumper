#!/bin/bash
set -e

echo "Running all omr-bumper tests..."
cd "$(dirname "$0")"

# Find all test directories
TEST_DIRS=$(find . -type d -name "????-*" | sort)

# Track whether any tests failed
FAILED=0

# Run each test
for test_dir in $TEST_DIRS; do
    echo ""
    echo "===================================================="
    echo "Running test: $test_dir"
    echo "===================================================="
    
    if [ -f "$test_dir/test_bumper.sh" ]; then
        pushd "$test_dir" > /dev/null
        if bash test_bumper.sh; then
            echo "✅ Test $test_dir PASSED"
        else
            echo "❌ Test $test_dir FAILED"
            FAILED=1
        fi
        popd > /dev/null
    else
        echo "⚠️  No test_bumper.sh found in $test_dir"
    fi
done

if [ $FAILED -eq 0 ]; then
    echo ""
    echo "✅ All tests passed successfully!"
    exit 0
else
    echo ""
    echo "❌ Some tests failed!"
    exit 1
fi