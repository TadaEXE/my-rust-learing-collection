use colored::Colorize;
use terminal_display_engine::TerminalDisplayEngine;

mod terminal_display_engine;
fn main() {
    let mut display = TerminalDisplayEngine::<30, 15>::new(8);
    let x: usize = 10;
    let y: usize = 10;

    let pxl = display.add_tracked_pixel(x, y, "x".white());
    loop {
        display.move_tracked_pixel(pxl, 1, 0);
        display.screen_refresh();
    }
}
