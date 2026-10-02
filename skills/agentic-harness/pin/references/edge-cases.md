# Pin Edge Cases

Responses and behaviours for situations the main skill file only summarises.

| Situation | Behaviour |
| --- | --- |
| Content already pinned (exact match on `content`) | Respond `⚠️ Already pinned.` and stop. Do not write the file. |
| Category already holds 5 items | Drop the oldest item of that type (lowest id), then append the new one. |
| Board already holds 20 items | Apply the per-type drop first; treat frequent hits on the limit as a sign the pins are too granular. |
| `/pin rm` with no number | Respond `⚠️ Usage: /pin rm <number>`. |
| `/pin rm <n>` where no item has that id | Respond `⚠️ Pin #N not found.` and leave the file unchanged. |
| `/pin show` on a missing or empty file | Respond `📋 Pin board is empty.` Do not return an error. |
| `/pin clear` | Reset items to empty but keep the current `next_id`. |
| Text contains ` — ` | Split into `content` (before) and `detail` (after). |
| Emoji prefix parsing | Split on the first emoji character, not the first space. |
| After any pin command | Resume the prior conversation exactly where it left off. |
