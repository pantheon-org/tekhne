# Scenario 2: Competitive Runner: Movement and Score Handlers

## User Prompt

A competitive runner game has had reports of cheaters teleporting across the map and submitting impossible scores. The lead developer suspects the current server blindly applies whatever coordinates and score the client sends in the move message. The team wants message handlers that are robust against manipulation.

Implement the `move` and `collectCoin` message handlers for the game room. Players send movement as a delta (change in x/y, not an absolute position). Collecting a coin should increment the player's score. The game runs at 30fps so move messages are very frequent.
