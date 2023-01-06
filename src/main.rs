use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use virus_blaster::{
    frame::{self, new_frame, Drawable},
    player::Player,
    render,
    vira::Vira,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "src/sounds/explode.wav");
    audio.add("lose", "src/sounds/lose.wav");
    audio.add("move", "src/sounds/move.wav");
    audio.add("pew", "src/sounds/pew.wav");
    audio.add("startup", "src/sounds/startup.wav");
    audio.add("win", "src/sounds/win.wav");
    audio.play("startup");

    // Initialize Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_sender, render_receiver) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match render_receiver.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut vira = Vira::new();
    'gameloop: loop {
        // Per-frame initialization
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut current_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if vira.update(delta) {
            audio.play("move");
        }

        // Draw and render
        // player.draw(&mut current_frame);
        // vira.draw(&mut current_frame);
        // use generics instead:
        let drawables: Vec<&dyn Drawable> = vec![&player, &vira];
        for drawable in drawables {
            drawable.draw(&mut current_frame);
        }
        let _ = render_sender.send(current_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Clean up:
    drop(render_sender);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
