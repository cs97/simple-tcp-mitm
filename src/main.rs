
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::Write;

use std::io::Read;
use std::net::Shutdown;
use std::thread;



fn tunnel(stream1: &mut TcpStream, stream2: &mut TcpStream) {
  loop {

    let mut buf = vec![0u8; 1024];

    match stream1.read(&mut buf) {
      Ok(n) if n > 0 => {
        stream2.write(&mut buf[..n]).unwrap();
        println!("{} -> {}", stream1.local_addr().unwrap(), stream2.local_addr().unwrap());
        println!("{}", String::from_utf8_lossy(&buf[..n]));
      },

      Ok(n) if n == 0 => break,
      Ok(_) => break,
      Err(_) => break,
    };

  }
}

fn main_foo() {

  let ip = "127.0.0.1";
  let port1 = "8080";	//listen
  let port2 = "6666";	//connect

  let addr = format!("{}{}{}", ip, ":", port1);
  let listener = TcpListener::bind(addr).unwrap();
  let (mut stream1, _addr) = listener.accept().unwrap();

  let addr2 = format!("{}{}{}", ip, ":", port2);
  let mut stream2 = TcpStream::connect(addr2).expect("no service");	//expect no service

  let mut stream1_clone = stream1.try_clone().expect("clone failed...");
  let mut stream2_clone = stream2.try_clone().expect("clone failed...");

  let thread1 = thread::spawn(move || {
    tunnel(&mut stream1_clone, &mut stream2_clone);
    let _ = stream1_clone.shutdown(Shutdown::Both);
    let _ = stream2_clone.shutdown(Shutdown::Both);
  });

  let thread2 = thread::spawn(move || {
    tunnel(&mut stream2, &mut stream1);
    let _ = stream1.shutdown(Shutdown::Both);
    let _ = stream2.shutdown(Shutdown::Both);
  });

  let _ = thread1.join();
  let _ = thread2.join();

}

fn main() {
  loop {
    main_foo();
  }
}
