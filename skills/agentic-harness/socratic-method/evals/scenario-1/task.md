# Vague Feature Request

## Problem Description

A developer comes to you with this request:

> "I want to add authentication to my app."

Use the socratic-method skill to work through this with them before writing any code.
The app is a small TypeScript/Express REST API with no existing auth layer.

The developer has not mentioned: which auth strategy (JWT, sessions, OAuth), who the users are,
what "secure enough" means for their use case, or whether they need multi-tenancy.

## Expected Behavior

- Activate socratic-method questioning mode
- Do NOT immediately scaffold an auth system
- Ask Phase 1 questions to establish what "authentication" means to them
- Surface at least two hidden assumptions (e.g. session vs. stateless, user types)
- After two rounds of dialogue, synthesize a refined, specific requirement
- Only then offer implementation options aligned to the clarified need

## Output Specification

- A Socratic dialogue transcript showing at least two question-answer rounds
- A synthesized requirement statement before any implementation begins
