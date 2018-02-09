# docker-attach
Interactively attach to running Docker containers, using Rust

## Installation
To build the binary, clone the repo and run `cargo build`. Then copy the `target/debug/docker-attach`
binary wherever you want.

## Usage
Just run `docker-attach`. It'll display an interactive list of running docker containers.
Use (`j`, `k`) to scroll, then `enter` to select an image to attach to. `Ctrl+c` or `Esc` quit the program.
