use colored::{ColoredString, Colorize};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub struct TerminalDisplayEngine<const X: usize, const Y: usize> {
    pixel_map: [[ColoredString; X]; Y],
    tracked_pixels: Vec<(usize, usize, ColoredString)>,
    last_refresh: Instant,
    fps: Duration,
}

impl<const X: usize, const Y: usize> TerminalDisplayEngine<X, Y> {
    pub fn new(fps: u64) -> Self {
        let tracked_pixels = Vec::new();
        let pixel_map: [[ColoredString; X]; Y] =
            core::array::from_fn(|_| core::array::from_fn(|_| " ".on_black()));
        let last_refresh = Instant::now();
        let fps = Duration::from_millis(1_000 / fps);

        TerminalDisplayEngine {
            pixel_map,
            tracked_pixels,
            last_refresh,
            fps,
        }
    }

    pub fn add_tracked_pixel(&mut self, x: usize, y: usize, pixle: ColoredString) -> usize {
        self.tracked_pixels.push((x, y, pixle.clone()));
        self.pixel_map[x][y] = pixle;
        self.tracked_pixels.len() - 1
    }

    pub fn remove_tracked_pixel(&mut self, pxl_id: usize) -> () {
        self.tracked_pixels.remove(pxl_id);
    }

    pub fn move_tracked_pixel(&mut self, pxl_id: usize, x_offset: usize, y_offset: usize) -> () {
        let old = self.tracked_pixels[pxl_id].clone();
        let new_x = (old.0 + x_offset) % X;
        let new_y = (old.1 + y_offset) % Y;
        self.tracked_pixels[pxl_id] = (new_x, new_y, old.2);
        self.pixel_map[old.1][old.0] = " ".on_black();
        self.pixel_map[new_y][new_x] = self.tracked_pixels[pxl_id].2.clone();
    }

    pub fn change_tracked_pixel(&mut self, pxl_id: usize, pixle: ColoredString) -> () {
        let old = self.tracked_pixels[pxl_id].clone();
        self.tracked_pixels[pxl_id] = (old.0, old.1, pixle.clone());
        self.pixel_map[old.1][old.0] = pixle;
    }

    pub fn get_pixel_info(&self, pxl_id: usize) -> (usize, usize, ColoredString) {
        self.tracked_pixels[pxl_id].clone()
    }

    pub fn screen_refresh(&mut self) -> () {
        for x in 0..Y {
            for y in 0..X {
                print!("{}", self.pixel_map[x][y]);
            }
            println!();
        }

        if self.fps > self.last_refresh.elapsed() {
            let delta_time = self.fps - self.last_refresh.elapsed();
            sleep(delta_time);
        }
        self.last_refresh = Instant::now();
    }
}
