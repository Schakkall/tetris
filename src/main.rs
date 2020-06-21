use utils;
use view::screen::show;

use std::thread;

pub fn main() {
    
    show(800, 600);    
    show(1024, 720);    

    std::thread::spawn( || {
        println!("Hello, Out of the window! {}", utils::sum(0,1));
    });
    
}
