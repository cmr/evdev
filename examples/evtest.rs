// Similar to the evtest tool.

use std::io::prelude::*;

fn main() {
    let mut args = std::env::args_os();
    let mut d = if args.len() > 1 {
        evdev::Device::open(&args.nth(1).unwrap()).unwrap()
    } else {
        let mut devices = evdev::enumerate().collect::<Vec<_>>();
        for (i, d) in devices.iter().enumerate() {
            println!("{}: {}", i, d.name().unwrap_or("Unnamed device"));
        }
        print!("Select the device [0-{}]: ", devices.len());
        let _ = std::io::stdout().flush();
        let mut chosen = String::new();
        std::io::stdin().read_line(&mut chosen).unwrap();
        devices.swap_remove(chosen.trim().parse::<usize>().unwrap())
    };
    println!("{}", d);
    println!("Events:");
    loop {
        for ev in d.fetch_events().unwrap() {
            println!("{:?}", ev);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
