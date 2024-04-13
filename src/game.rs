use rand::{rngs::ThreadRng, Rng};
use std::io::{Read, Write};

pub struct Game<R, W> {
    pub width: u32,
    pub height: u32,
    pub glyph: char,
    pub game_state: u32, // 0 = play, 1 = idle, 2 = stop
    pub world: Vec<Vec<bool>>,
    pub player1: (u8, u8, u8),
    pub player2: (u8, u8, u8),
    pub stdin: R,
    pub stdout: W,
    pub rand: ThreadRng,
}

const X: [i32; 8] = [0, 1, 0, -1, 1, 1, -1, -1];
const Y: [i32; 8] = [-1, 0, 1, 0, -1, 1, 1, -1];

impl<R: Read, W: Write> Game<R, W> {
    fn rand_color(&mut self) -> (u8, u8, u8) {
        (
            self.rand.gen_range(0..255),
            self.rand.gen_range(0..255),
            self.rand.gen_range(0..255),
        )
    }

    pub fn new(width: u32, height: u32, glyph: char, stdin: R, stdout: W) -> Self {
        let world = vec![vec![false; width as usize]; height as usize];

        let rand = rand::thread_rng();

        let player1 = (0, 0, 0);
        let player2 = (0, 0, 0);

        let game_state = 0;

        Self {
            width,
            height,
            glyph,
            world,
            game_state,
            player1,
            player2,
            stdin,
            stdout,
            rand,
        }
    }

    fn init(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Hide,
            termion::clear::All
        )
        .unwrap();

        self.game_state = 0;

        self.player1 = self.rand_color();
        self.player2 = self.rand_color();

        // half the width is false and half is true
        self.world = (0..self.height)
            .map(|_| (0..self.width).map(|x| x < self.width / 2).collect())
            .collect();
    }

    fn handle_input(&mut self) {
        let mut buf = [0; 1];
        if self.stdin.read(&mut buf).is_ok() {
            match buf[0] {
                b'c' => {
                    self.player1 = self.rand_color();
                    self.player2 = self.rand_color();
                }
                b' ' => self.game_state = 0,
                b's' => self.game_state = 1,
                b'q' => self.game_state = 2,
                _ => {}
            }
        }
    }

    pub fn run(&mut self) {
        self.init();

        let dt: f32 = 1.0 / 30.0;
        let mut last = std::time::Instant::now();
        let mut acc: f32 = 0.0;
        'main_loop: loop {
            acc += last.elapsed().as_secs_f32();
            last = std::time::Instant::now();

            while acc >= dt {
                self.handle_input();

                match self.game_state {
                    0 => {}
                    1 => {
                        continue 'main_loop;
                    }
                    2 => {
                        break 'main_loop;
                    }
                    _ => {}
                }

                self.update(dt);

                acc -= dt;
            }

            self.draw();

            if self.check_end() {
                if self.restart() {
                    self.init();
                    continue 'main_loop;
                }

                break 'main_loop;
            }
        }

        write!(
            self.stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::style::Reset,
            termion::cursor::Show,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn restart(&mut self) -> bool {
        self.stdout.flush().unwrap();
        self.game_state = 1;

        loop {
            let mut buffer = [0];
            self.stdin.read(&mut buffer).unwrap();

            match buffer[0] {
                b'r' => return true,
                b'q' => return false,
                _ => {}
            }
        }
    }

    fn check_end(&self) -> bool {
        return self
            .world
            .iter()
            .all(|row| row.iter().all(|cell| *cell == true))
            || self
                .world
                .iter()
                .all(|row| row.iter().all(|cell| *cell == false));
    }

    pub fn update(&mut self, _dt: f32) {
        let mut new_world = self.world.clone();

        for (y, row) in self.world.iter().enumerate() {
            for (x, _cell) in row.iter().enumerate() {
                let neighbors: Vec<usize> = (0..8)
                    .filter(|i| {
                        let nx = x as i32 + X[*i];
                        let ny = y as i32 + Y[*i];
                        nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32
                    })
                    .collect();

                let mut count = 0;
                for i in neighbors.iter() {
                    count += self.world[(y as i32 + Y[*i]) as usize][(x as i32 + X[*i]) as usize]
                        as usize;
                }

                let ratio = count as f32 / neighbors.len() as f32;
                let rng = self.rand.gen_range(0.0..1.0);

                new_world[y][x] = ratio > rng;
            }
        }

        self.world = new_world;
    }

    pub fn draw(&mut self) {
        for (y, row) in self.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let color = if *cell { self.player1 } else { self.player2 };

                write!(
                    self.stdout,
                    "{}{}{}",
                    termion::cursor::Goto(x as u16 + 1, y as u16 + 1),
                    termion::color::Fg(termion::color::Rgb(color.0, color.1, color.2)),
                    self.glyph
                )
                .unwrap();
            }
        }

        // display settings at the bottom
        write!(
            self.stdout,
            "{}{}q = quit, r = replay, c = change colors, s = stop, space = play{}",
            termion::cursor::Goto(1, self.height as u16 + 1),
            termion::color::Fg(termion::color::Rgb(255, 255, 255)),
            termion::style::Reset
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }
}
