# Pokemon AI

A WIP bot that is able to play pokemon. Not much progress so far, just simple i/o and some partial parsing of the Pokemon map

## Dependencies 
This project uses a fork of an old build of mooneye-gb with some tools I wrote for scripting. Maybe I should have used the C bindings for a more popular emulator like mGBA or SameBoy or libgamette, but this is where we are right now & I'm decently familiar with Rust and the mooneye codebase.


## Configuration and running

```
cargo build --release
```

Use the WARP_SPEED env variable to speed up the game. WARP_SPEED = 1 is normal speed. I think this can go up to about 16.

## Reference materials
https://github.com/pret/pokered
