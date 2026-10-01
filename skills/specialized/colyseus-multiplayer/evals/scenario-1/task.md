# Scenario 1: Battle Arena: Player State Design

## User Prompt

A small studio is building a 4-player battle arena game with Colyseus. Each player needs several tracked attributes: health points, x/y position, an active status flag, and a score. The team's previous prototype had a bug where spectators saw stale health values and score updates were invisible to opponents — a clear state sync issue they need to avoid this time.

Design the Schema classes for the room. The room should support up to 4 players joining by session. When a player joins, they should be added to the room state with default values. When a player leaves, they should be removed.
