use std::io::{Read, Write, ErrorKind, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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
    fn handle(&self, stream: TcpStream) -> std::io::Result<()>;
}

impl TcpConnectionStrategy for ThreadPerConnection {
    fn handle(&self, mut stream: TcpStream) -> std::io::Result<()> {
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

                // println!("{}", String::from_utf8_lossy(&buffer));
                // stream.write(&buffer).unwrap();
            }
        });

        Ok(())
    }
}

struct TcpServerConfig {
    strategy: Box<dyn TcpConnectionStrategy>
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
            if let Err(error) = self.config.strategy.handle(stream?) {
                println!("Error: {}", error);
            }
        }

        Ok(())
    }
}


fn main() {
    let server = TcpServer::new(TcpServerConfig {
        strategy: Box::new(ThreadPerConnection::new(1))
    });

    server.listen(8080).unwrap();
}
