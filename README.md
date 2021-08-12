## Description

This is the source code of my game Panda Doodle, which can be played at [https://pandadoodle.lucamoller.com/](https://pandadoodle.lucamoller.com/) (it's best playable on touch screen devices).

For more information, check out the article I wrote ([Rewriting my mobile game in Rust targeting WASM](https://lucamoller.medium.com/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830)) describing the background, the motivation and my experience while working on this project.

This code was written for fun and originally for my eyes only. I tried to clean/organize things up a bit (believe me it was much messier) before sharing, but there's so much more that could be done. Also, this code was not intended to be "rusty". On the contrary I was trying to see far I could go by writing non-rusty code in Rust (I talk more about this in the [article]((https://lucamoller.medium.com/rewriting-my-mobile-game-in-rust-targeting-wasm-1f9f82751830)) mentioned above). So please forgive me if something hurts your eyes :)

For someone else trying to develop games with Rust+WASM, I think the most useful part of this repository is the engine module. It contains the basic functionality for interacting with the browser APIs (like rendering on Canvas2d and input) and also basic frameworks/utilities the game relies on for animations,UI, etc. The code there is fully independent of the game logic and doesn't depend on any game specific logic/types (I guess I could have extracted it in a separate crate). It's not a generic engine with a broad set of features though, it contains exactly the features the required by the game, which were implemented as needed haha.

This repository does not contain all assets to build the functioning Panda Doodle game (static resources such as images and sounds and also level descriptions were left out), but it contains all the Rust/JS code used by the game with hopes that someone can learn/reuse something from it. 


## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `html` folder contains the index.html file which is the landing page in which the game runs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build.

* The `tests` folder contains your Rust unit tests.
