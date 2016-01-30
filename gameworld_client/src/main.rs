use std::io;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::str;

fn main() {
	foo().unwrap();
}

fn foo() -> Result<(), io::Error> {

	println!("Got here!");
	let socket = try!(UdpSocket::bind("127.0.0.1:34253"));
	let dest_ip_addr = Ipv4Addr::new(127, 0, 0, 1);
	let dest_sock_addr = SocketAddrV4::new(dest_ip_addr, 34254);

	let buf = "Zilch".as_bytes();
	println!("Sending...");
	try!(socket.send_to(&buf, &dest_sock_addr));

	//println!("Got here!3");
	//let (amt, src) = try!(socket.recv_from(&mut buf));
	//let mut resp = [0; 10];

	//println!("Got here!4");
	//try!(socket.recv_from(&mut resp));

	//println!("Got here!5");
	//println!("{}", str::from_utf8(&resp).unwrap());

	//println!("Got here!6");
	//drop(socket); // close the socket

	println!("Sent! I think...");
	Ok(())
}
