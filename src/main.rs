extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;


pub fn main() {
    //thread::spawn(|| {
        let sdl_context = sdl2::init().expect("SDL initialization failed!");
        let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem.");

        let window = video_subsystem.window("Tetris", 800, 600)
                    .position_centered()
                    .opengl()
                    .build()
                    .expect("Failed to create window.");
        
        let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump!");

        let mut canvas = window.into_canvas().build().expect("Failed to convert window into canvas.");
                                    
       
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => { 
                        break 'running
                    },
                    Event::KeyDown {keycode: Some(Keycode::Escape), ..} =>
                    {
                        break 'running
                    },
                    Event::KeyDown {keycode: Some(Keycode::Return), ..} =>
                    {                        
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                        canvas.clear();                  
                        canvas.present();                        
                    },
                    _ => {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.clear();                  
                        canvas.present();                                       
                    }
                }
            }            
            
        }        
        
    //});
    

    println!("Hello, Out of the window!");

    
    //thread::sleep(Duration::from_millis(10000000));
}
