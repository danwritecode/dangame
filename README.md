# dangame

A multiplayer 2D fighting/platformer game built in Rust, for fun, using [macroquad](https://github.com/not-fl3/macroquad) for rendering and [renet](https://github.com/lucaspoffo/renet) for netcode.

## What is this?

This is a hobby project where I’m learning and experimenting with real-time multiplayer game development in Rust. The goal is to have a simple, smooth, and fun 2D game where you can run, jump, and fight with friends over the network. It’s not meant to be a commercial product—just a playground for game dev ideas, networking, and Rust code.

## Features

- **Real-time multiplayer**: Server-authoritative, UDP-based netcode using renet.
- **Smooth movement**: Client-side interpolation for remote players, so things don’t look jittery.
- **Multiple characters**: Each with their own animations and moves.
- **Simple maps**: Tiled backgrounds and platforms.
- **Menu UI**: Pick your character, map, and connect to a server.

## High-Level Design Choices

- **macroquad**: Chosen for its simplicity and async-friendly API. It’s easy to get something on screen and iterate quickly.
- **renet**: Handles the UDP networking and reliable/unreliable channels, so I can focus on game logic.
- **Shared types in `common/`**: All the data structures that go over the network (like player state) are defined in one place and shared between client and server.
- **Separation of concerns**:  
  - The client (`game/`) handles rendering, input, and local simulation.
  - The server (`server/`) is authoritative and just keeps track of all player states.
- **Interpolation, not prediction**: For now, remote players are interpolated between their last two known positions for smoothness. No client-side prediction yet, but maybe in the future.
- **Trait-based character logic**: All characters implement a common trait, so adding new ones is easy.

## How it works

- The client sends its position and animation state to the server at a fixed interval.
- The server keeps a map of all connected clients and their latest state.
- The server broadcasts updates to all clients at a fixed interval (only when something changes).
- Each client interpolates remote players’ positions for smooth movement.
- The main game loop is single-threaded (macroquad limitation), but networking is rate-limited so it doesn’t bog down the frame rate.

## Running

You’ll need Rust (nightly might be required for some crates).

```sh
# In one terminal, start the server:
cd server
cargo run

# In another terminal, start the client:
cd game
cargo run
```

You can run multiple clients to test multiplayer locally, or connect over LAN/internet by changing the server address in the menu.
