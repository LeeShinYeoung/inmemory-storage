use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

trait TcpConnectionStrategy {
    fn handle(&self, listener: TcpListener);
}

struct ConnectionPool;

impl TcpConnectionStrategy for ConnectionPool {
    fn handle(&self, listener: TcpListener) {

        let connection = Arc::new(Mutex::new(0));

        for stream in listener.incoming() {
            let connection = Arc::clone(&connection);

            {
                let mut connection = connection.lock().unwrap();
                if *connection > 1 {
                    println!("Connection #{} rejected", connection);
                    continue;
                }

                *connection += 1;
            }

            let mut stream = stream.unwrap();

            thread::spawn(move || {
                loop {
                    let mut buffer = [0; 512];
                    let byte = stream.read(&mut buffer).unwrap();

                    if byte == 0 {
                        let mut connection = connection.lock().unwrap();
                        println!("Connection #{} closed", connection);
                        *connection -= 1;
                        break;
                    }

                    println!("{}", String::from_utf8_lossy(&buffer));
                    stream.write(&buffer).unwrap();

                    {
                        let connection = connection.lock().unwrap();
                        println!("Connection #{}", connection);
                    }
                }
            });
        }
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
        self.config.strategy.handle(listener);

        Ok(())
    }
}


fn main() {
    let server = TcpServer::new(TcpServerConfig {
        strategy: Box::new(ConnectionPool)
    });

    server.listen(8080).unwrap();
}
