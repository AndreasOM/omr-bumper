# omr-bumper Tests

This directory contains tests for the omr-bumper tool.

## Test Structure

Each test is in its own directory with a numeric prefix for ordering:

- `0001-simple`: Basic functionality test
- `0002-untracked-files`: Test handling of untracked files

## Setting Up Test Repositories

Before running tests, you need to set up the test Git repositories:

```bash
./setup_test_repos.sh
```

This script creates Git repositories and remote repositories for each test directory. These repositories are excluded from the main project's Git repository via `.gitignore`.

## Running Tests

To run all tests:

```bash
./run_all_tests.sh
```

To run a specific test:

```bash
cd 0001-simple
./test_bumper.sh
```

## Adding New Tests

To add a new test:

1. Create a new directory with the next available number prefix (e.g., `0003-new-test`)
2. Copy the basic structure from an existing test
3. Modify the test script to test the specific functionality
4. Run `./setup_test_repos.sh` to initialize the Git repository structure

Each test should be self-contained and clean up after itself.