extern crate ws;
extern crate env_logger;

use ws::listen;

fn main() {
    // Setup logging
    env_logger::init().unwrap();

    println!("Data backend running at `127.0.0.1:12345`.");
        
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("127.0.0.1:12345", |out| {

        // The handler needs to take ownership of out, so we use move
        move |msg| {

            // Handle messages received on this connection
            println!("Server got message '{}'. ", msg);

            // Use the out channel to send messages back
            out.send(msg)
        }

    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

fn works() -> bool {
   return false
}
