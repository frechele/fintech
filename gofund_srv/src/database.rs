use tokio_postgres::{Client, NoTls};

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;

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

    pub async fn get_avail_tickers(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for row in self
            .client
            .query("SELECT DISTINCT code FROM correlation", &[])
            .await
            .unwrap()
        {
            let code: &str = row.get(0);
            result.push(code.to_string());
        }

        result
    }

    pub async fn get_value(
        &mut self,
        value_type: &str,
        code: &str,
        end_date: &str,
        limit: i32,
    ) -> HashMap<String, f64> {
        let mut result: HashMap<String, f64> = HashMap::new();

        let query = format!(
            "SELECT date::TEXT, {} FROM \"{}\" WHERE date <= '{}' ORDER BY date DESC LIMIT {}",
            value_type, code, end_date, limit
        );

        for row in self.client.query(&query, &[]).await.unwrap() {
            let date: String = row.get(0);
            let close: f64 = row.get(1);
            result.insert(date, close);
        }

        result
    }
}
