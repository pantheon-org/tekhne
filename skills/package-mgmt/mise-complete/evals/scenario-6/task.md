# Scenario 6: Consolidate nvm and pyenv Version Files into Mise

## User Prompt

A monorepo has `.nvmrc` (Node 20.11.0), `.python-version` (3.11.7), and a CI script that runs `nvm use` and `pyenv local` before every job. Two sub-packages each carry their own copy of `.nvmrc`. Move the project to Mise. Show the files you would produce and the order of your steps, including how you would deal with the existing managers.
