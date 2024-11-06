/// Configure the server.

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub threads: i8,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        // Set the default host & port
        let host = "127.0.0.1".to_string();
        let port = "8080".to_string();
        let threads: i8 = 4;

        match args.len() {
            1 => {
                println!("> No arguments provided, using default values");
                return Config {
                    host: host,
                    port: port,
                    threads: threads,
                };
            }
            2 => {
                println!("> No Port or Threads provided, using default value");
                return Config {
                    host: args[2].clone(),
                    port: port,
                    threads: threads,
                };
            }
            3 => {
                println!("> Host and Port args provided, using default for Threads");
                return Config {
                    host: args[2].clone(),
                    port: args[3].clone(),
                    threads: threads,
                };
            }
            4 => {
                return Config {
                    host: args[2].clone(),
                    port: args[3].clone(),
                    threads: args[4]
                        .clone()
                        .parse::<i8>()
                        .expect("Threads must be a number!"),
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
