# Scenario 1: A Vague Goal Is Not Recorded, It Is Questioned

## User Prompt

"Goal for today: sort out the GitLab situation."

Repo state: the goals directory is empty. No goal file exists yet.

## Expected Behavior

1. Run the clarity test before writing anything (Workflow step 1).
2. Check 1 fails: "sort out the GitLab situation" contains nothing checkable, so nothing
   distinguishes done from not done.
3. Because check 1 failed, route to `socratic-method`, not `guided-interview`. The problem
   itself is undefined, so concrete options would be guesses.
4. Do not create a goal file. Do not invent a `## Done looks like` sentence on the user's
   behalf, and do not enumerate items they did not state.
5. Ask an open question aimed at what a finished GitLab situation would look like.

## Success Criteria

- No goal file is created in this turn.
- The response does not present an invented list of items as if the user had stated them.
- The response routes to, or asks questions in the manner of, `socratic-method` rather than
  offering a multiple-choice option set.
- The reason given is that the goal has no verifiable end state, not merely that it is "vague".
- At most a small number of questions are asked, rather than a questionnaire.

## Failure Conditions

- Writes a goal file with a placeholder or guessed end state.
- Routes to `guided-interview` and presents 3 to 4 curated options when the problem itself is
  undefined.
- Silently starts work on whatever GitLab issue seems most likely.
- Records the goal anyway and notes that it should be refined later.
