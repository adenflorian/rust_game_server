#![allow(dead_code)]

extern crate rand;
extern crate rustc_serialize;

mod actor;
mod vector2;
mod socket;

use actor::Actor;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;

fn main() {

	let mut rx: Receiver<String> = start_server_thread();

	println!("GAME LOOP START");

	let mut actors: Vec<Actor> = Vec::new();

	actors.push(Actor::new(String::from("Aden")));
	actors.push(Actor::new(String::from("Bundt")));
	actors.push(Actor::new(String::from("Chen")));
	actors.push(Actor::new(String::from("Danny")));
	actors.push(Actor::new(String::from("Evan")));
	actors.push(Actor::new(String::from("Felix")));
	actors.push(Actor::new(String::from("Gary")));
	// actors.push(Actor::new(String::from("Howie")));
	// actors.push(Actor::new(String::from("Ian")));
	// actors.push(Actor::new(String::from("Jack")));
	// actors.push(Actor::new(String::from("Karl")));
	// actors.push(Actor::new(String::from("Lex")));
	// actors.push(Actor::new(String::from("Mun")));

	let mut quit = false;

	let mut counter = 0;
	let max_counter = 5;

	actors_say_positions(&actors);

	let mut actors_clone: Vec<Actor>;
	let mut count;

	loop {
		// TODO This is a problem
		// allows Actors to move on top of eachother
		actors_clone = actors.clone();
	    for actor in &mut actors {
	        actor.think(&actors_clone);
	    }

	    // TODO Take input from clients
	    // If something from a client
	    // Then push new actor
	    count = 0;
	    loop {
	        match rx.try_recv() {
			Ok(m) => {
				println!("main loop: pushing new actor...");
				actors.push(Actor::new(m));
				//rx = start_server_thread();
				count += 1;
			},
				_ => {println!("Received {} messages", count); break},
			};
	    }
		

	    // TODO Make Board struct

	    // TODO Send updated positions to connected clients
	    // for client in &connected_clients {
	    //     client.send_actors(&actors);
	    // }

	    counter += 1;

	    if counter == max_counter {
	        quit = true;
	    }

	    if quit {
	    	actors_say_positions(&actors);
	        break;
	    }
	}
	println!("GAME LOOP END");
}

fn actors_say_positions(actors: &Vec<Actor>) {
	for actor in actors {
        actor.say_position();
    }
}

pub struct World {
	inhabitants: Vec<Actor>,
}

impl World {
    pub fn new() -> World {
    	World {
    		inhabitants: Vec::new(),
    	}
    }
}

fn start_server_thread() -> Receiver<String> {
	let (tx, rx) = channel();

	println!("Spawning server thread...");
	thread::spawn(move || {
		println!("Hi from newly spawned server thread!");

		
		loop {
		    let new_actor_name: String = socket::do_socket().unwrap();
			println!("Received String: {}", new_actor_name);
			tx.send(new_actor_name).unwrap();
		}


		drop(tx);
	});

	rx
}
