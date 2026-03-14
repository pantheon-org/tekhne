# evals task.md
Create a `/deploy` slash command that:
1. Accepts an environment name as an argument (e.g. `/deploy staging`)
2. Runs `git status` to check for uncommitted changes first
3. Injects the current git log (last 5 commits) into the prompt
4. Runs the appropriate deploy script for the specified environment

Place the command file in the correct location.
