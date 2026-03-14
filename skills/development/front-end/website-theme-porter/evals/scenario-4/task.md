# Extract Design Tokens from a React App

A client has asked you to port the visual theme from their existing React marketing site at `https://app.client-brand.io` into a new project. You have confirmed that `agent-browser` is available in your environment.

The site is a single-page React application — its content is rendered client-side after JavaScript executes.

Write a shell script called `extract.sh` that performs the complete Stage 1 extraction for this site. The script should:
- Set up the artifact directory
- Navigate to the URL, handle the SPA rendering behaviour appropriately, and take screenshots
- Extract the CSS custom properties and computed element styles and save them to the artifact directory

Use `app-client-brand-io` as the website slug and `2026-03-13` as the date.

The script does not need to be runnable in this environment — write it as if you would execute it against the live site.
