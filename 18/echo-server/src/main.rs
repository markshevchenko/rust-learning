use std::net::TcpListener;
use std::io;
use std::thread::spawn;

fn echo_main(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("listening on {}", address);

    loop {
        let (mut stream, address) = listener.accept()?;

        println!("request from {}", address);

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream).expect("Error");
            println!("Connection closed");
        });
    }
}

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}
