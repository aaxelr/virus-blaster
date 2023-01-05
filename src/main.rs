use rusty_audio::Audio;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "src/sounds/explode.wav");
    audio.add("lose", "src/sounds/lose.wav");
    audio.add("move", "src/sounds/move.wav");
    audio.add("pew", "src/sounds/pew.wav");
    audio.add("startup", "src/sounds/startup.wav");
    audio.add("win", "src/sounds/win.wav");
    audio.play("startup");

    // Clean up: block until all audio is done playing.
    audio.wait();
    Ok(())
}
