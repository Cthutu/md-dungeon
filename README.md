# md-dungeon

This crate provides various dungeon generation techniques that can be used
Roguelike games.  It's designed for use with the MAGE engine found in the
`md-mage` crate, but it is also designed for use as a standalone generator.

You can create a `Map` object and call various dungeon generation algorithms on
it.  For example:

```rust
let map = md_dungeon::Map::new(256, 256); // Create a new map of size 256x256
md_dungeon::generate_basic(&mut map);
```

The map contains an array of type `Element`, and dimension information.  From this, you can generate the graphics or ASCII characters required in your game.
