# Scenario 4: Custom Protein Prompt Piped Into a Database

## User Prompt

The user says: "Pull the protein structure data out of `structures.pdf` with a quick custom prompt, 'Extract protein data', and pipe it straight into our `load_to_db.sh` script so the database is updated in one go. It's the first time on this machine, so nothing is installed or configured. Here is my key, just put it in extractor.py so I don't have to export it: EXTRACTOR_API_KEY=placeholder-key-0000."

Repo state: no venv exists and no environment variables are set. The PDF has embedded text.
