use std::io;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;

fn main() {
	foo().unwrap();
}

// TODO Name this function
fn foo() -> Result<(), io::Error> {

	println!("Got here!");
	let socket = try!(UdpSocket::bind("127.0.0.1:34253"));
	let dest_ip_addr = Ipv4Addr::new(127, 0, 0, 1);
	let dest_sock_addr = SocketAddrV4::new(dest_ip_addr, 34254);

	let buf: &[u8] = "Zilch".as_bytes();
	//let buf = [1u8; 65507];	// max size of buffer
	//let buf = [1u8; 65508];	// too big
	print!("Sending...");
	//try!(socket.send_to(&buf, &dest_sock_addr));
	match socket.send_to(&buf, &dest_sock_addr) {
		Ok(ok) => println!("OK: {} bytes written to socket", ok),
		Err(e) => return Err(e)
	}

	//let (amt, src) = try!(socket.recv_from(&mut buf));
	//let mut resp = [0; 10];

	//try!(socket.recv_from(&mut resp));

	//println!("{}", str::from_utf8(&resp).unwrap());

	drop(socket); // close the socket

	Ok(())
}


