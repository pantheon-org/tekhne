# Commit Message Review and Correction

## Problem Description

A junior developer on your team has been submitting pull requests with poorly formatted git commit messages. The tech lead has asked you to review a batch of proposed commit messages and produce corrected versions for each one, along with a brief explanation of what was wrong.

The messages come from a mix of work: a new export feature added to the API, a fix for a race condition in the session manager, some test coverage added for the payment module, and a CI pipeline update for deployment automation.

The developer's raw proposed messages are listed below. For each one, produce a corrected commit message and note what was fixed.

## Output Specification

Create a file called `commit-corrections.md` that contains:
- Each original message (quoted)
- The corrected commit message
- A one-sentence explanation of what was wrong

## Input Files

The following proposed messages need correction. Extract them before starting.

=============== FILE: inputs/proposed-messages.txt ===============
1. Added new export endpoint for user data to the REST API
2. chore: fixed null pointer crash when session token is missing
3. Updated tests for payment processing to cover edge cases
4. CI pipeline changes for blue-green deployment
5. Refactored the authentication module by extracting token validation logic into a separate helper function and updating all callers
