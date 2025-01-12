use std::net::TcpListener;
use std::io::Read;
fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Sever is listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection established");

                let mut buffer = [0; 1024];

                stream.read(&mut buffer)?;

                let request = String::from_utf8_lossy(&buffer[..]);
                println!("Recieved: {}", request);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}