use std::net::SocketAddrV4;
use actor::Actor;
use std::io;
use std::net::UdpSocket;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct Client {
	address: SocketAddrV4
}

impl Client {
	pub fn new(address: SocketAddrV4) -> Client {
		Client {
			address: address
		}
	}
    pub fn send_actors(&self, actors: &Vec<Actor>) -> Result<(), io::Error> {
		let socket = try!(UdpSocket::bind("127.0.0.1:33353"));

		let mut message_vec: Vec<u8> = Vec::new();

		for actor in actors {
			let actor_name = actor.get_name();
			let actor_name_bytes = actor_name.as_bytes();
			for byte in actor_name_bytes {
		    	message_vec.push(byte.clone());
			}

			// Use byteorder to turn a Vector2 into bytes
			let actor_position = actor.get_position();
	    	message_vec.write_i32::<LittleEndian>(actor_position.x).unwrap();
	    	message_vec.write_i32::<LittleEndian>(actor_position.y).unwrap();
		}

		println!("message vector length: {}", message_vec.len());

		print!("Sending...");

		match socket.send_to(&message_vec, &self.address) {
			Ok(ok) => println!("OK: {} bytes written to socket", ok),
			Err(e) => return Err(e)
		}

		drop(socket); // close the socket

		Ok(())
    }
}
