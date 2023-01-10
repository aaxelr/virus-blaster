# Virus Blaster

Basic Space Invaders like game that runs in the terminal.

Built as a part of Nathan Stocks' [Ultimate Rust Crash Course](https://www.udemy.com/course/ultimate-rust-crash-course/).


## Running the game
Run the game by cloning the repository, `cd` into the directory and run:
```shell
> cargo run
```
You can also build an executable and run it:
```shell
> cargo build
> target/debug/virus_blaster
```
To build an optimised executable, add the `--release` flag and run the release version:
```shell
> cargo build --release
> target/release/virus_blaster
```

## How To Play
`Left Arrow`: move left

`Right Arrow`: move right

`Space`: Fire space laser

`Esc` or `Q`: Quit