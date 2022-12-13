use tokio_postgres::{Client, NoTls};

use serde::ser::{Serialize, SerializeStruct, Serializer};

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
    )
    .await
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

pub struct Correlation {
    code: String,
    term: i32,
    corr: f64,
}

impl Serialize for Correlation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Correlation", 3)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("term", &self.term)?;
        state.serialize_field("corr", &self.corr)?;
        state.end()
    }
}

impl Database {
    pub async fn get_correlations(&mut self) -> Vec<Correlation> {
        let mut result: Vec<Correlation> = Vec::new();
        for row in self
            .client
            .query("SELECT * FROM correlation", &[])
            .await
            .unwrap()
        {
            let code: &str = row.get::<_, &str>(0);
            let term: i32 = row.get(1);
            let corr: f64 = row.get(2);

            result.push(Correlation {
                code: code.to_string(),
                term: term,
                corr,
            });
        }

        result
    }
}
