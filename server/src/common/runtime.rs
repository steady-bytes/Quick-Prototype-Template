use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::env;

use dotenv::dotenv;
use clap::{Parser, Subcommand};
use sea_orm::{DatabaseConnection};

use crate::common::router;

///////////////////////////////
/// ******* RUNTIME ******* ///
///////////////////////////////

///
pub struct Runtime {
    socket_address: Option<SocketAddr>,
    _database_connection: Option<DatabaseConnection>,
}

type RuntimeResult<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub struct RuntimeError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Settings {
    #[clap(subcommand)]
    runtime: Mode,
}

#[derive(Subcommand, Debug)]
enum Mode {
  Server,
  Client,
}

impl Runtime {
    pub fn new () -> Runtime {
        Runtime { 
            socket_address: None, 
            _database_connection: None,
        }
    }

    // read default env vars
    pub async fn default(&self) -> RuntimeResult<Runtime> {
        dotenv().ok();

        let args = Settings::parse();

        match args.runtime {
            Mode::Server => {
                Ok(self.server().await.unwrap())
            },
            Mode::Client => {
                // TODO ->  replace all of the make targets with commands in the client.
                // this makes for an amazing dev experience.
                Ok(self.client().unwrap())
            }
        }
    }

    pub fn client(&self) -> RuntimeResult<Runtime> {
        Err(RuntimeError)
    }

    pub async fn server(&self) -> RuntimeResult<Runtime> {
        let _db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
        let port = env::var("PORT").expect("PORT environment variable not set");
        let ip = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
        let socket_address = SocketAddr::new(ip, port.parse::<u16>().unwrap());
        // let database_connection = Database::connect(&db_url).await.unwrap();
        
        Ok(Runtime {
            socket_address: Some(socket_address), 
            _database_connection: None,
        })
    }

    pub async fn execute(self) {
        let app = router::new();
        let listener = self.socket_address.unwrap();
        let _ = axum::Server::bind(&listener).serve(app.into_make_service()).await;
    }    
}