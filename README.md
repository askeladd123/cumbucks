# Cumbucks

A time management program with a twist; instead of planning what to do, and when to take a break, the program rolls a dice and decides for you. 

This is a GUI-version of [cumbucks-terminal](https://github.com/askeladd123/time-for-socks-terminal).

## Run
> You can run by clicking this link: [github pages](https://askeladd123.github.io/cumbucks/).

## Build for Web
To build the app as a website first you need:
- the [rust toolchain](https://www.rust-lang.org/tools/install)
- ***wasm32-unknown-unknown*** target installed
  - run `rustup target add wasm32-unknown-unknown`
- [trunk](https://crates.io/crates/trunk) bundler
  - run `cargo install trunk`

Then:
- `trunk serve --release`
- *trunk* will make a local server
  - use the app on [localhost:1420]() in a browser

## Build for Native
To build as a desktop app you need:
- [tauri](https://tauri.app/v1/guides/getting-started/prerequisites) bundler
  - run `cargo install tauri-cli`
- run `cargo tauri dev`
  - *tauri-cli* will open the app in a desktop window *(and serve a website at [localhost:1420]())*
---

## Problems
### github pages / workflow
Trunk routes files based on links, but github pages expects repo name to be included in link [thedodd, issue](https://github.com/thedodd/trunk/issues/268). This means I can't automatically deploy with github actions without changing *index.html*. I fixed this by making a script [trunk-gh-fixer](/trunk-gh-fixer.py).
