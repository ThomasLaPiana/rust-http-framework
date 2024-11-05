/// Configure the server.

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        // Set the default host & port
        let host = "127.0.0.1".to_string();
        let port = "8080".to_string();

        match args.len() {
            1 => {
                println!("> No arguments provided, using default values");
                return Config {
                    host: host,
                    port: port,
                };
            }
            2 => {
                println!("> No Port provided, using default value");
                return Config {
                    host: args[2].clone(),
                    port: port,
                };
            }
            3 => {
                println!("> Host and Port args provided");
                return Config {
                    host: args[2].clone(),
                    port: args[3].clone(),
                };
            }
            _ => {
                panic!("Too many arguments provided");
            }
        }
    }

    pub fn addr(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
