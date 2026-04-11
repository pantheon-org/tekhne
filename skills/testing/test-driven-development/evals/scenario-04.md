# Scenario 04: File Processing Queue System

## User Prompt

DataFlow Inc. processes thousands of customer data files daily through their analytics platform. Currently, files are processed synchronously, causing timeouts and blocking other operations when large files are uploaded. The engineering team needs an asynchronous queue system that can handle file processing jobs efficiently.

The system should queue incoming files, process them in the background, and update job status as files move through different processing stages (uploaded, processing, completed, failed). Users should be able to check the status of their file processing jobs and receive notifications when processing is complete.

The challenge is that the existing system has been experiencing intermittent test failures due to timing issues and race conditions in the asynchronous processing logic. The team needs a robust solution with reliable tests that don't suffer from these timing problems.

Build a file processing queue system with:

1. Job queue management (add jobs, get status, process queue)
2. Asynchronous file processing with status updates
3. Error handling for processing failures
4. Job completion notifications

Key requirements:
- Handle multiple jobs concurrently
- Provide real-time status updates
- Gracefully handle processing errors
- Support different file types (CSV, JSON, XML)

Required files:
- Queue system implementation
- File processor implementation
- Job status tracking
- Comprehensive test suite with proper async testing
- Example usage demonstrating concurrent job processing

## Expected Behavior

1. Have tests not use `setTimeout`, `sleep`, or fixed delays to wait for async operations
2. Use proper async test patterns like `await expect().resolves`, `waitFor`, or promise-based assertions
3. Control timing in tests through mocks, promises, or synchronization mechanisms — not arbitrary waits
4. Maintain clear Arrange-Act-Assert structure with proper `await` usage in async tests
5. Properly clean up async operations in each test so they don't affect other tests
6. Test async error scenarios using proper rejection handling — not `try/catch` patterns
7. Have tests pass consistently without timing dependencies or race conditions
8. Mock external async services to eliminate network/IO timing variability
9. Name tests to clearly describe the async behavior being tested

## Success Criteria

- **No arbitrary sleeps**: Tests do not use `setTimeout`, `sleep`, or fixed delays to wait for async operations
- **Proper async assertions**: Uses proper async test patterns like `await expect().resolves`, `waitFor`, or promise-based assertions
- **Deterministic timing**: Tests control timing through mocks, promises, or synchronization mechanisms, not arbitrary waits
- **AAA structure in async**: Async tests maintain clear Arrange-Act-Assert structure with proper `await` usage
- **Isolated async tests**: Each test properly cleans up async operations and doesn't affect other tests
- **Error case handling**: Tests async error scenarios using proper rejection handling, not `try/catch` patterns
- **Stable test execution**: Tests pass consistently without timing dependencies or race conditions
- **Mock async dependencies**: External async services are mocked to eliminate network/IO timing variability
- **Clear async test names**: Test names describe the async behavior being tested clearly

## Failure Conditions

- Tests use `setTimeout`, `sleep`, or hard-coded delays to wait for async completion
- Tests use `.then()` chaining or callbacks instead of `await`/`async` patterns
- Timing in tests is controlled by arbitrary millisecond values rather than synchronization mechanisms
- Async tests do not follow Arrange-Act-Assert structure
- Async operations leak between tests, causing interference
- Async error cases are tested with `try/catch` blocks rather than proper rejection assertions
- Tests fail intermittently due to timing or race conditions
- External async services are not mocked, causing tests to depend on real I/O timing
- Test names do not describe the async behavior or expected outcome
