# Scenario: Refactor an impure function to pure

A function reads `Date.now()` internally, making it non-deterministic. Refactor it to accept the timestamp as a parameter.
