use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

pub struct Virus {
    pub x: usize,
    pub y: usize,
}

pub struct Vira {
    pub army: Vec<Virus>,
    move_timer: Timer,
    direction: i32,
}

impl Vira {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < NUM_ROWS / 2)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Virus { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|virus| virus.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|virus| virus.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for virus in self.army.iter_mut() {
                    virus.y += 1;
                }
            } else {
                for virus in self.army.iter_mut() {
                    virus.x = ((virus.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|virus| virus.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_virus_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(i) = self
            .army
            .iter()
            .position(|virus| (virus.x == x) && (virus.y == y))
        {
            self.army.remove(i);
            true
        } else {
            false
        }
    }
}

impl Drawable for Vira {
    fn draw(&self, frame: &mut Frame) {
        for virus in self.army.iter() {
            frame[virus.x][virus.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "M"
            } else {
                "W"
            }
        }
    }
}
