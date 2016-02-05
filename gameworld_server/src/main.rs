#![allow(dead_code)]

extern crate rand;
extern crate byteorder;

mod actor;
mod vector2;
mod socket;
mod client;
mod world;

use actor::Actor;
use client::Client;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

fn main() {

	let rx: Receiver<String> = start_server_thread();

	println!("GAME LOOP START");

	let mut actors: Vec<Actor> = Vec::new();

	init_actors(&mut actors);

	let mut quit = false;

	let mut counter = 0;
	let max_counter = 3;

	actors_say_positions(&actors);

	let mut actors_clone: Vec<Actor>;

	let mut connected_clients: Vec<Client> = Vec::new();

	let dest_ip_addr = Ipv4Addr::new(127, 0, 0, 1);
	let dest_sock_addr = SocketAddrV4::new(dest_ip_addr, 31222);
	connected_clients.push(Client::new(dest_sock_addr));

	let mut recvd_msg_count;

	loop {
		// TODO This is a problem
		// allows Actors to move on top of eachother
		actors_clone = actors.clone();
	    for actor in &mut actors {
	        actor.think(&actors_clone);
	    }

	    // If something from a client
	    // Then push new actor
	    recvd_msg_count = 0;
	    loop {
	        match rx.try_recv() {
			Ok(m) => {
				println!("main loop: pushing new actor...");
				actors.push(Actor::new(m));
				//rx = start_server_thread();
				recvd_msg_count += 1;
			},
				_ => {println!("Received {} messages", recvd_msg_count); break},
			};
	    }
		

	    // TODO Make Board or World struct

	    // TODO Send updated positions to connected clients
	    for client in &connected_clients {
	        client.send_actors(&actors).unwrap();
	    }

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
	});

	return rx;
}

fn init_actors(actors: &mut Vec<Actor>) {
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
}
