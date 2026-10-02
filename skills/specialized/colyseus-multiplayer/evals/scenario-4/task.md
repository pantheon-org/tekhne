# Scenario 4: Party Game Lobby: Host Controls

## User Prompt

A party game needs a lobby room where the first player to join becomes the host. The host should have exclusive abilities: starting the match when enough players have joined, and removing a disruptive player from the lobby. Other players should not be able to trigger either of these actions, even if they try to send the same messages.

Implement the lobby room with host tracking and the two privileged message handlers: one to start the match and one to remove a specific player by their session ID.
