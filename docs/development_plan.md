x - Check for gamepads on startup and add to player
x - Map left stick movements to player model
x - Follow player with camera.
x - Map right stick movements to camera for player.
- Pin vertical camera movement to +- pi/2. Right now the player can continuously spin vertically around the character.
x - Make player movement relative to character and not the world. For example up on the left stick always sends the player "north". I think that should send the player "forward" no matter their orientation in the world coordinate system.
- Rotate the player character entity based on movement.
- Add controller dead zones.


- add tests
- Rework player spawning. Spawn player on controller connection and despawn player on
  controller disconnection. This way a player can drop in.
