# Scenario 06: Performance Analysis: Memory Limits, Flush Intervals, and Storage Configuration

## User Prompt

An infrastructure team running a high-throughput analytics platform has experienced repeated Fluent Bit OOM-kills and log delivery delays during peak hours. Their current configuration was set up by a contractor and has never been tuned for production load. The team wants a performance-focused analysis before their next product launch.

Analyze the Fluent Bit configuration below for performance issues. Identify missing memory limits, problematic flush intervals, missing storage configuration, and any other settings that would cause instability under high log volume. Produce a corrected configuration and a performance analysis report.
