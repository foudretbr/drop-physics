# Horizon Physics

A native mod for [Horizon](https://playhorizon.dev) that gives dropped items some physics.

- A dropped item tumbles while it flies.
- On the ground it settles on its flattest side: flat items lie on their face, blocks on a side.
- No more bobbing or spinning on the ground.

It only changes how items are drawn (client side). Nothing changes in the world or for other players.

## Credits

The idea and the approach of the item drawing come from
[Horizon Physics by tariksabanuz](https://github.com/tariksabanuz/PhysicsModHorizon) (MIT), itself inspired by
PhysicsMod by Haubna. This is a smaller rewrite for the current Horizon API; the tumbling and settling code is new.
The game's own item drawing code, which the mod follows, is Horizon's.

## Licence

MIT, see `LICENSE`.
