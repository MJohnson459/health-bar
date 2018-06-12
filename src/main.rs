extern crate blinkt;
extern crate palette;

use std::thread::sleep;
use std::time::Duration;

use palette::{Gradient, Lch, LinSrgb, Pixel, Srgb};
use blinkt::Blinkt;

fn main() {
    println!("Hello, world!");
    let mut blinkt = Blinkt::new().expect("Unable to create Blinkt device");

    let grad1 = Gradient::new(vec![
        LinSrgb::new(1.0, 0.1, 0.1),
        LinSrgb::new(0.1, 0.1, 1.0),
        LinSrgb::new(0.1, 1.0, 0.1),
    ]);

    let mut health = 0.0; // between 0.0 and 1.0
    let mut diff = 0.1;
    let pixels = 8.0;

    loop {
        println!("");
        for (i, color) in grad1.take((pixels * health) as usize).enumerate() {
            println!("{} = ({}, {}, {})", i, (255.0 * color.red) as u8, (255.0 * color.green) as u8, (255.0 * color.blue) as u8);
            // blinkt.set_pixel(i, (255.0 * color.red) as u8, (255.0 * color.green) as u8, (255.0 * color.blue) as u8)
        }

        blinkt.show().expect("Unable to print pixels");

        health += diff;
        if health > 1.0 {health = 1.0; diff = -diff;} ;
        if health < 0.0 {health = 0.0; diff = -diff;} ;
        sleep(Duration::from_millis(200));
    }
}
