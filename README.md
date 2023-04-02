# Rusty Snake

This is a simple implementation of the classic Snake game in Rust, using the Rust programming language, based on this article: https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html

## How to Play

The objective of the game is to guide the snake to eat food while avoiding collisions with the walls or the snake's own tail. The game ends when the snake collides with a wall or its own tail.

Use the arrow keys on your keyboard to control the movement of the snake. The snake will continue to move in the direction it is currently facing until it collides with a wall or its own tail, or until you change its direction by pressing one of the arrow keys.

## Installation

1. Make sure you have Rust and Cargo installed on your system. If you don't have them installed, you can download and install them from the official Rust website: https://www.rust-lang.org/tools/install.

2. Clone this repository to your local machine.

Copy code repository
```
$ git clone https://github.com/filipmuntean/rusty-snake.git
```

3. Change into the project directory.
```
$ cd snake-in-rust
``` 

4. Build and run the game using Cargo.
```
$ cargo run
```

## Dependencies

This project uses the following dependencies:

crossterm - This Rust library offers easy-to-use functionality for event (input) handling, content styling, cursor manipulation, and performing terminal actions on both Windows and UNIX systems.
rand - Rand offers a suite of tools to generate random numbers, convert them to useful types and distributions, as well as several algorithms related to randomness.
