use std::io;
use std::net::UdpSocket;
use std::str;

pub fn do_socket() -> Result<String, io::Error> {
    let socket = try!(UdpSocket::bind("127.0.0.1:34254"));

	let mut buf = [0; 10];
	let (amt, src) = try!(socket.recv_from(&mut buf));

	// Send a reply to the socket we received data from
	let buf = &mut buf[..amt];

	let new_actor_name: &str = str::from_utf8(&buf).unwrap();
	println!("socket: Received: {}", new_actor_name);
	
	//let resp_str: &str = "Thanks!!!!!!!!!";
	//let resp_arr: &[u8] = resp_str.as_bytes();
	//try!(socket.send_to(resp_arr, &src));

	drop(socket); // close the socket

	Ok(String::from(new_actor_name))
}
