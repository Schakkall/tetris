use utils;
use view::screen::show;

use std::thread;

pub fn main() {
    
    //std::thread::spawn(move || {
        show(800, 600);    
    //});

    println!("Hello, Out of the window! {}", utils::sum(0,1));
    
}
