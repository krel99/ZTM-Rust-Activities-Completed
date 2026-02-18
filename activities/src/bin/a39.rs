// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    ChangeColor(u8, u8, u8),
    On,
    Off,
    Disconnect,
}

enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    thread::spawn(move || {
        let mut status = LightStatus::Off;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!(
                            "Color changed to ({}, {}, {}). Light is {}.",
                            r,
                            g,
                            b,
                            match status {
                                LightStatus::On => "on",
                                LightStatus::Off => "off",
                            }
                        );
                    }
                    LightMsg::On => {
                        println!("Light turned on");
                        status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Light turned off");
                        status = LightStatus::Off;
                    }
                    LightMsg::Disconnect => {
                        println!("Disconnecting. Light off.");
                        status = LightStatus::Off;
                        return status;
                    }
                },
                Err(_) => {
                    println!("Channel disconnected. Light off.");
                    status = LightStatus::Off;
                    return status;
                }
            }
        }
    })
}

fn main() {
    let (s, r) = unbounded();

    let light = spawn_light_thread(r);

    s.send(LightMsg::On).expect("failed to send");
    s.send(LightMsg::ChangeColor(255, 0, 0))
        .expect("failed to send");
    s.send(LightMsg::ChangeColor(0, 255, 0))
        .expect("failed to send");
    s.send(LightMsg::Off).expect("failed to send");
    s.send(LightMsg::ChangeColor(0, 0, 255))
        .expect("failed to send");
    s.send(LightMsg::Disconnect).expect("failed to send");

    let final_status = light.join().expect("failed to join light thread");
    match final_status {
        LightStatus::Off => println!("Final status: light is off"),
        LightStatus::On => println!("Final status: light is on"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
