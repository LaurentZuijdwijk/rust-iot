extern crate ws;

use self::ws::{Handler, Sender, Result, Message, Handshake, CloseCode, Error, Settings};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::Cell;


struct Server {
    clients:Arc<Mutex<HashMap<u32, Sender>>>,
    out: Sender,
    count: Rc<Cell<u32>>,
    id : u32,
}
trait HasBroadcast {
	fn broadcast(&self, msg: Message)->Result<()>;
}
impl HasBroadcast for Server {
	fn broadcast(&self, msg:Message)->Result<()> {
        for (key, client) in self.clients.lock().unwrap().iter(){
        	if *key != self.id {
	        	// println!("{:?} {:?}", *key, self.id);
	        	client.send(msg.clone());
	        	client.send(format!("hello {} from {}",key, &self.id));
       		}
        }
        return Ok(());
	}
}


pub fn create() {

  println!("0.0.0.0:3012");
  let count = Rc::new(Cell::new(0));
  let uuid = Rc::new(Cell::new(100));
  let map:HashMap<u32, Sender> = HashMap::new();
  let mutex_map = Arc::new(Mutex::new(map));
  ws::Settings.max_connections = 200;
  let ws = ws::listen("0.0.0.0:3012", |out| {
  	uuid.set(uuid.get()+1);
    let id = uuid.get();

  	 Server { out: out, count: count.clone(), clients:mutex_map.clone(), id:id }
  }).unwrap();

} 


impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.clients.lock().unwrap().insert(self.id.clone(), self.out.clone());
        
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("The number of live connections is {}", self.count.get());
        self.broadcast(msg)?;
        return Ok(())
//        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.clients.lock().unwrap().remove(&self.id.clone());

        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}
