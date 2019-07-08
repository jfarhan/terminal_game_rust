//extern crate termion;
//use termion::color;
use slow_game::graphics::Screen;

fn main() {
    println!("Hello, world!");

    let mut screen_one=Screen::new_base_block(640,480);
    screen_one.show_screen();

}








