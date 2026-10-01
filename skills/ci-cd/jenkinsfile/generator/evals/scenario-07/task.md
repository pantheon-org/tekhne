# Scenario 07: Cross-Platform Library Test Matrix

## User Prompt

You are building a test pipeline for a Java library that must be validated against three JDK versions (11, 17, 21) and two operating systems (Linux and Windows), producing six total build combinations. All combinations must be tested, and if any combination fails the remaining ones should also be cancelled. After the matrix completes, a test summary stage should publish results.
