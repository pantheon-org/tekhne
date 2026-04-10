# Scenario 01: Vague Feature Request

## User Prompt

A developer comes to you with this request:

> "I want to add authentication to my app."

Use the socratic-method skill to work through this with them before writing any code.
The app is a small TypeScript/Express REST API with no existing auth layer.

The developer has not mentioned: which auth strategy (JWT, sessions, OAuth), who the users are,
what "secure enough" means for their use case, or whether they need multi-tenancy.

## Expected Behavior

1. Activate socratic-method questioning mode without scaffolding any auth code or proposing an implementation
2. Open Phase 1 by establishing what "authentication" means in this specific context and who the users are
3. Surface at least two hidden assumptions (e.g., session vs. stateless, user types) before proposing any solutions
4. Ask no more than three questions per turn at any point in the dialogue
5. After two rounds of dialogue, synthesise a specific, scoped requirement statement before offering implementation options

## Success Criteria

- **Enters questioning mode without scaffolding**: Enters questioning mode without scaffolding any auth code or proposing an implementation
- **Phase 1 establishes context**: Phase 1 establishes what 'authentication' means in context and who the users are
- **Surfaces hidden assumptions**: Surfaces at least two hidden assumptions (e.g. session vs stateless, user types) before proposing solutions
- **Three questions per turn max**: Asks no more than three questions per turn at any point in the dialogue
- **Synthesises before implementing**: Synthesizes a specific, scoped requirement statement before offering implementation options

## Failure Conditions

- Immediately scaffolds an auth system or proposes JWT/OAuth before asking any clarifying questions
- Skips Phase 1 and jumps straight to implementation options
- Asks more than three questions in a single turn
- Proposes implementation options before surfacing at least two hidden assumptions
- Never synthesises a requirement statement — goes from questions directly to code
