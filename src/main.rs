use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

trait TcpConnectionStrategy {
    fn handle(&self, stream: TcpStream);
}

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

    fn open_connection(active_connections: &Arc<Mutex<u32>>, max_connections: u32) {
        let mut active_connections = active_connections.lock().unwrap();

        if *active_connections >= max_connections {
            println!("Max connections reached");
            return;
        }

        *active_connections += 1;
    }

    fn close_connection(active_connections: &Arc<Mutex<u32>>) {
        let mut active_connections = active_connections.lock().unwrap();

        if *active_connections == 0 {
            println!("No connections to close");
            return;
        }

        *active_connections -= 1;
        println!("Connection #{} closed", active_connections);
    }
}

impl TcpConnectionStrategy for ThreadPerConnection {

    fn handle(&self, mut stream: TcpStream) {
        let connection = Arc::clone(&self.active_connections);

        ThreadPerConnection::open_connection(&connection, self.max_connections);

        thread::spawn(move || {
            loop {
                let mut buffer = [0; 512];
                let byte = stream.read(&mut buffer).unwrap();

                if byte == 0 {
                    ThreadPerConnection::close_connection(&connection);
                    break;
                }

                println!("{}", String::from_utf8_lossy(&buffer));
                stream.write(&buffer).unwrap();
            }
        });
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
            self.config.strategy.handle(stream?);
        }

        Ok(())
    }
}


fn main() {
    let server = TcpServer::new(TcpServerConfig {
        strategy: Box::new(ThreadPerConnection::new(3))
    });

    server.listen(8080).unwrap();
}
