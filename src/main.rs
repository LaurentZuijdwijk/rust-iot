mod websock;
mod rpi;
use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    if !cfg!(target_os = "macos") {
        let (_, _) = rpi::gpio::create().unwrap();
        //    tx.send("aaaaa").unwrap();
        //    println!("sending {}", "aaaa");
    }

    websock::conn::create().unwrap();


}
