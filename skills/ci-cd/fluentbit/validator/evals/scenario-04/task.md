# Scenario 04: Tag Routing: Detecting Unmatched INPUT Tags That Silently Drop Logs

## User Prompt

An SRE team has been investigating a "missing logs" incident in production. Their Kubernetes logging pipeline appeared healthy — Fluent Bit was running, no errors in the Fluent Bit pod logs — but application logs from their payment service were simply not appearing in their log aggregation backend. After two hours of debugging, they suspect a tag routing misconfiguration.

Validate the Fluent Bit configuration below with a focus on tag routing. Trace every INPUT tag through all FILTER and OUTPUT Match patterns. Identify any INPUT tags that are not covered by an OUTPUT Match, any FILTER Match patterns that do not correspond to INPUT tags, and any orphaned sections. Produce a corrected configuration and a routing analysis report.
