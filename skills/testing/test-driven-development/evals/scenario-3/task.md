# File Processing Queue System

## Problem/Feature Description  

DataFlow Inc. processes thousands of customer data files daily through their analytics platform. Currently, files are processed synchronously, causing timeouts and blocking other operations when large files are uploaded. The engineering team needs an asynchronous queue system that can handle file processing jobs efficiently.

The system should queue incoming files, process them in the background, and update job status as files move through different processing stages (uploaded, processing, completed, failed). Users should be able to check the status of their file processing jobs and receive notifications when processing is complete.

The challenge is that the existing system has been experiencing intermittent test failures due to timing issues and race conditions in the asynchronous processing logic. The team needs a robust solution with reliable tests that don't suffer from these timing problems.

## Output Specification

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

**Required files:**

- Queue system implementation
- File processor implementation
- Job status tracking
- Comprehensive test suite with proper async testing
- Example usage demonstrating concurrent job processing
