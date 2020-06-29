use utils;
use view::screen::show;

use std::thread;
use std::time::Duration;

use std::result::Result;

pub fn main() {
    
    show(736, 360);//Smartphone    
    show(720, 1024);//Wide screen    

    std::thread::spawn( || {    
        let is_sum_an_error: Result<u32, &str> = Ok(utils::sum(0,1));
        assert_eq!(is_sum_an_error.is_err(), false);
    
        println!("Hello, Out of the window! {} ->a<-sserting", utils::sum(0,1));        
        thread::sleep(Duration::from_millis(6000));      
    });
    
}
