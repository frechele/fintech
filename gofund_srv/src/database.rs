use tokio_postgres::{Client, NoTls};

use crate::config::{load_configuration, Config};

pub struct Database {
    client: Client,
}

pub async fn connect_db() -> Database {
    let config: Config = load_configuration("config.toml");

    connect_db_impl(
        &config.database.host,
        &config.database.user,
        &config.database.password,
        &config.database.dbname,
    ).await
}

async fn connect_db_impl(host: &str, user: &str, password: &str, dbname: &str) -> Database {
    let connection_url = format!("postgresql://{}:{}@{}/{}", user, password, host, dbname);

    let (client, connection) = tokio_postgres::connect(&connection_url, NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Database { client }
}

impl Database {
    pub async fn execute(&mut self) {
        for row in self.client.query("SELECT * FROM correlation", &[]).await.unwrap() {
            let code: &str = row.get::<_, &str>(0);
            let term: i32 = row.get(1);
            let corr: f64 = row.get(2);

            println!("{} {} {}", code, term, corr);
        }
    }
}
