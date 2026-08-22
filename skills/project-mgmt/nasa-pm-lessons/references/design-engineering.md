# Design Engineering

Paraphrased from Jerry Madden, NASA GSFC — "Lessons Learned for Project Managers" (LLIS #1956), <https://llis.nasa.gov/lesson/1956>. Consult the source for Madden's original wording and full
elaboration.

Use during root-cause analysis, incident writeups, and code/design review — pick the 1-3 that fit, don't dump the list.

- "Previously flown" hardware is a fiction — different builders, minor changes, a different operating environment, and a test crew that doesn't fully understand the unit, every single time.
- Equipment works as built, not as designed — layout quirks and misunderstood component specs win over the designer's intent.
- On a failure: build a timeline of the known facts first, test every theory against it, and know when to stop forcing a scenario onto the data.
- Redundancy can be an illusion — identical builds tend to fail identically. Treat every unit as one-of-a-kind and mission-critical.
- Don't fear failure, but build the skill and the network to recover from it.
- Testing beats experience — proving beats assuming something will work.
- Reviews exist for the reviewed, not the reviewer. A review that teaches the reviewed nothing has failed.
- A heavy review and reporting burden signals that management doesn't understand the work — keep material simple enough for someone only slightly familiar to follow.
- Hands-on engineering knowledge has eroded; often only the tooling "knows for sure" now, and it isn't explaining itself.
- Mistakes are recoverable; failure is a mistake you couldn't recover from. Build contingency plans for the high-risk items.
- A fixed-price culture makes requirements creep a cardinal sin, unlike the earlier push-the-limits era.
- Review load keeps growing while knowledge transfer doesn't — design reusable material that survives being reshuffled between presentations.
- When something works, study why and try to repeat it.
- The schedule and cost curves signal trouble before engineers admit it — engineers are natural optimists.
- External reviews land at the worst possible time — keep technical data current so you can respond fast without a scramble.
- Hide nothing from reviewers. State the facts; skip the excuses.
- Review bureaucracies entrench themselves once established — work with the system rather than against it.
- Test results can mislead — computer models hide flaws, often traceable to bad input data.
- Software now carries hardware's burdens (creep, cost share, QA) plus the extra difficulty of proving it isn't flawed. Get the core working first, keep working fallback versions, and have a
  contingency plan.
- Every project hits a parts problem eventually, no matter how much qualification testing preceded it — stay ready to react.
- Problems are seeded at the very start. Most failures trace back to weak initial planning, not a single bad moment later on.
- Talking to the right people beats most other diagnostics. Silence at the right level is dangerous.
- Never decide from a diagram or cartoon alone — look at the real hardware or the underlying data.
- Space failures usually trace to the commonplace, overlooked item, not the hard task that got done well.
- Reviews, meetings, and reality often have little in common with each other.
