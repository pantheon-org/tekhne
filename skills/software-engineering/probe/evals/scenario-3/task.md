# Scenario 3: Interpreting Probe Results and Deciding Next Step

## User Prompt

"The probe results are in. We tested whether the rate limiter correctly handles burst traffic. Observations: the first 10 requests in a burst succeed, requests 11-15 return 429, but request 16 unexpectedly succeeds and subsequent requests also succeed — the limiter seems to reset at an unexpected point."
