use std::io::{Read, Write, ErrorKind, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

struct ThreadPerConnection {
    active_connections: Arc<Mutex<u32>>,
    max_connections: u32
}

impl ThreadPerConnection {
    fn new(max_connections: u32) -> Self {
        ThreadPerConnection {
            active_connections: Arc::new(Mutex::new(0)),
            max_connections
        }
    }

    fn open_connection(active_connections: &Arc<Mutex<u32>>, max_connections: u32) -> std::io::Result<()> {
        let mut active_connections = active_connections.lock().unwrap();

        if *active_connections >= max_connections {
            return Err(Error::new(ErrorKind::ConnectionRefused, "Max connections reached"));
        }

        *active_connections += 1;
        Ok(())
    }

    fn close_connection(active_connections: &Arc<Mutex<u32>>) {
        let mut active_connections = active_connections.lock().unwrap();

        if *active_connections == 0 {
            println!("No connections to close");
            return
        }

        *active_connections -= 1;
    }
}

trait TcpConnectionStrategy {
    fn handle(&self, stream: TcpStream, sender: Sender<[u8; 512]>) -> std::io::Result<()>;
}

impl TcpConnectionStrategy for ThreadPerConnection {
    fn handle(&self, mut stream: TcpStream, sender: Sender<[u8; 512]>) -> std::io::Result<()> {
        let active_connection = Arc::clone(&self.active_connections);

        ThreadPerConnection::open_connection(&active_connection, self.max_connections)?;

        thread::spawn(move || {
            loop {
                let mut buffer = [0; 512];
                let byte = stream.read(&mut buffer).expect("Failed to read from stream");

                if byte == 0 {
                    ThreadPerConnection::close_connection(&active_connection);
                    break;
                }

                sender.send(buffer).unwrap();
            }
        });

        Ok(())
    }
}

struct TcpServerConfig {
    strategy: Box<dyn TcpConnectionStrategy>,
    sender: Sender<[u8; 512]>
}

struct TcpServer {
    config: TcpServerConfig
}

impl TcpServer {
    fn new(config: TcpServerConfig) -> TcpServer {
        TcpServer {
            config
        }
    }

    fn listen(&self, port: u16) -> std::io::Result<()> {
        let host = "localhost";
        let address = format!("{}:{}", host, port);
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            let sender = self.config.sender.clone();
            if let Err(error) = self.config.strategy.handle(stream?, sender) {
                println!("Error: {}", error);
            }
        }

        Ok(())
    }
}


fn main() {
    let (tx, rx): (Sender<[u8; 512]>, Receiver<[u8; 512]>) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            println!("Message: {}", String::from_utf8_lossy(&message));
        }
    });

    let server = TcpServer::new(TcpServerConfig {
        strategy: Box::new(ThreadPerConnection::new(1)),
        sender: tx
    });

    server.listen(8080).unwrap();
}
