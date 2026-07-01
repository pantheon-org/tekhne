# Apply Writing Style Best Practices

A junior developer contributed a draft `CONTRIBUTING.md` for an open-source project.
The content is correct but the writing style violates several best practices:
passive voice, wordy phrases, vague instructions, and inconsistent tone.

Here is the input document (`CONTRIBUTING.md`):

```markdown
# Contributing

## How to Make Contributions

In order to be able to contribute to this project, you will first need to
install the development dependencies on your local machine. The dependencies
should be installed by running the appropriate command.

```
npm install
```

After the dependencies have been installed, the test suite should be run in
order to verify that everything is working properly:

```
npm test
```

## Submitting Changes

When changes are ready to be submitted, it is necessary to create a pull
request. The PR template should be filled out completely.

It is important to note that all code changes must have accompanying tests.
Changes will not be accepted if tests are not passing.

## Code Style

The project's coding conventions should be followed at all times. ESLint is
used for code linting and Prettier is used for code formatting. It is
recommended that you configure your editor to run these tools automatically.

## Need Help?

If you should encounter any issues, do not hesitate to reach out to the team.
One can open a discussion on GitHub Discussions for assistance.
```

Produce a corrected `CONTRIBUTING.md` that:

1. Uses active voice throughout.
2. Removes wordy phrases like "In order to", "It is important to note that", "it is necessary to".
3. Replaces vague instructions with specific commands.
4. Adds explicit language tags to all fenced code blocks.
5. Addresses the reader directly using imperative mood or "you".
6. Does not change the document structure, headings, or technical meaning.
