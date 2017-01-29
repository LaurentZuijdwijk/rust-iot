extern crate cupi;
mod websock;
use std::thread;
use std::sync::mpsc::channel;
use cupi::{board, CuPi, delay_ms, DigitalWrite};
use std::time::Duration;

fn main() {
  let board = board().unwrap();
   println!("{:?}", board);
	
	let (tx, rx) = channel();

	
	let guard =  thread::spawn( move || {
	  loop {
  	  	if let Ok(msg) = rx.try_recv() {
   	       println!("message {:?}", msg);
			let cupi = CuPi::new().unwrap();
		    //let mut pinout = cupi.pin(0).unwrap().high().output();
		    let mut pin = cupi.pin_sys(0).unwrap();
		    pin.export().unwrap();
		    let mut pinout = pin.output().unwrap();

		    for _ in 0..20 {
		        pinout.high().unwrap();
		        delay_ms(600);
		        pinout.low().unwrap();
		        delay_ms(600);
		    }
	  	}
	  }

    });

thread::sleep(Duration::from_millis(4000));
  tx.send("aaaaa").unwrap();
  println!("sending {}", "aaaa");

  websock::conn::create();
  guard.join().unwrap();

} 