
#[cfg(feature = "rpi")]
extern crate cupi;
#[cfg(feature = "rpi")]
use cupi::{board, CuPi, delay_ms, DigitalWrite};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};


pub fn create() -> Option<(Sender<()>, Receiver<()>)> {
    #[cfg(feature = "rpi")]
    let doSetup = || {

        let board = board().unwrap();
        println!("{:?}", board);

        let (tx, rx) = channel();


        let guard = thread::spawn(move || {
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
        return (tx, rx);
    };
    #[cfg(feature = "rpi")]
    return Some(doSetup());
    return None;

}
