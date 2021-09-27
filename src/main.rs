extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;

extern crate clap;
use clap::{Arg, App};

// dispayed in an unreachable state
const MESSAGE: &str = "Hello, if your reading this, my program is in an what a believed is an unreachable state. \nreport this here https://github.com/10maurycy10/vsync/issues";

pub fn main() -> Result<(), String> {

       let matches = App::new("Vsync tester")
                          .version("0.1.0")
                          .author("Maurycy Z. <10maurycy10@gmail.com>")
                          .about("A program to test vsync from sdl2")
                          .arg(Arg::with_name("frames")
                               .short("f")
                               .long("frames")
                               .value_name("INTEGER")
                               .help("Sets the amount of frames for a test. suppored values are 2 and 3")
                               .default_value("2")
                               .takes_value(true))
                        .arg(Arg::with_name("no-vsync")
                               .short("n")
                               .long("no-vsync")
                               .help("Disables asking for vsync"))
                            .get_matches();

    let frames = matches.value_of("frames").expect(MESSAGE).parse::<i32>().expect("frames must be a number");
    
    let no_vsync = matches.occurrences_of("no-vsync");
    
    match frames {
        2 | 3 => {},
        _ => {
            println!("frames must be 2 or 3");
            return Err("frames must be 2 or 3".to_string());
        }
    }
    
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("vsync", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = match no_vsync {
        0 =>    window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?,
        _ =>    window.into_canvas().build().map_err(|e| e.to_string())?
    };

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{
                    ..
                } => break 'running,
                _ => {}
            }
        }

        match frames {
            3 => match i%3 {
                0 => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                1 => canvas.set_draw_color(Color::RGB(0, 255, 0)),
                2 => canvas.set_draw_color(Color::RGB(0, 0, 255)),
                _ => panic!("{}", MESSAGE)
            },
            2 => match i%2 {
                0 => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                1 => canvas.set_draw_color(Color::RGB(0, 255, 255)),
                _ => panic!("{}", MESSAGE)
            },
             _ => panic!("{}", MESSAGE)
        }
        
        canvas.clear();
        canvas.present();
//        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        i += 1;
    
    }

    Ok(())
}
