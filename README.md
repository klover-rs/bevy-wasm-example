# bevy-wasm-example
This is a simple example of a bevy game which is just a colorful ball on your screen.
You can move the ball with `WASD` and change to a random background color with `space` 

## How to compile for web assembly
1. `cargo  build --release --target wasm32-unknown-unknown`
2. `wasm-bindgen --no=typescript --target web --out-dir ./out/ --out-name "mygame" ./target/wasm32-unknown-unknown/release/browser_game.wasm`

and then you can run it with an http server of your choice!
