use utils;
use view::screen::show;

use std::thread;
use std::time::Duration;

pub fn main() {
    
    show(736, 360);//Smartphone    
    show(720, 1024);//Wide screen    

    std::thread::spawn( || {
        thread::sleep(Duration::from_millis(6000));      
        println!("Hello, Out of the window! {}", utils::sum(0,1));        
    });
    
}
