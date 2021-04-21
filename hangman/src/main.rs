use console_engine::screen::Screen;
use console_engine::Color;

fn main() {
    const SCREEN_WIDTH: u32 = 50;
    const SCREEN_HEIGHT: u32 = 20;

    let mut screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    screen.clear();

    screen.print_fbg(11, 4, "The Hangman - By Eiberham", Color::Red, Color::Reset);

    screen.draw();
}
