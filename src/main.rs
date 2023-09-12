use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

trait TcpConnectionStrategy {
    fn handle(&self, stream: TcpStream);
}

struct ConnectionPool;

impl TcpConnectionStrategy for ConnectionPool {
    fn handle(&self, mut stream: TcpStream) {
        loop {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            println!("{}", String::from_utf8_lossy(&buffer));
            stream.write(&buffer).unwrap();
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

        for stream in listener.incoming() {
            self.config.strategy.handle(stream?);
        }

        Ok(())
    }
}


fn main() {
    let server = TcpServer::new(TcpServerConfig {
        strategy: Box::new(ConnectionPool)
    });

    server.listen(8080).unwrap();
}
