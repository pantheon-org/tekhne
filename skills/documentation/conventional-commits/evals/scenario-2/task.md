# Write a Commit Message for a Performance Fix

## Problem Description

Your team has been investigating a slow endpoint in the search service. After profiling, an engineer discovered that the `/search` endpoint was executing an unbounded database query on every request, with no caching layer. Under sustained load the average response time was 480ms with a 99th percentile of 2.1 seconds.

The fix adds Redis caching with a 5-minute TTL and limits the maximum result set to 100 records. After the change, benchmarks show an average response time of 45ms and a 99th percentile of 190ms under the same load.

The change touches `services/search.ts`, `config/redis.ts`, and the query builder. The Redis connection pool is initialized lazily to avoid startup overhead.

Write a commit message for this change that will be used in the project's auto-generated CHANGELOG and will help future engineers understand why this optimization was made.

## Output Specification

Create a file called `commit-message.txt` containing the complete commit message.
