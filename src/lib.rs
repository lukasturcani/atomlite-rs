use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::{Executor, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bonds {
    pub atom1: Vec<u32>,
    pub atom2: Vec<u32>,
    pub order: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AromaticBonds {
    pub atom1: Vec<u32>,
    pub atom2: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Molecule {
    pub atomic_numbers: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atom_charges: Option<Vec<i8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonds: Option<Bonds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dative_bonds: Option<Bonds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aromatic_bonds: Option<AromaticBonds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformers: Option<Vec<Vec<[f32; 3]>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub key: String,
    pub molecule: Molecule,
    pub properties: Map<String, Value>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to connect to database")]
    Connect(#[source] sqlx::Error),
    #[error("transaction error")]
    Transaction(#[source] sqlx::Error),
}

pub struct Database {
    pool: SqlitePool,
    molecule_table: String,
}

impl Database {
    pub async fn connect(path: &str) -> Result<Database, Error> {
        Self::connect_with_table(path, "molecules").await
    }

    pub async fn connect_with_table(path: &str, molecule_table: &str) -> Result<Database, Error> {
        let pool = SqlitePool::connect(path).await.map_err(Error::Connect)?;
        sqlx::query(&format!(
            r#"
                CREATE TABLE IF NOT EXISTS {} (
                    key TEXT PRIMARY KEY NOT NULL,
                    molecule JSON,
                    properties JSON NOT NULL
                )
            "#,
            molecule_table
        ))
        .execute(&pool)
        .await
        .map_err(Error::Transaction)?;
        Ok(Database {
            pool,
            molecule_table: molecule_table.into(),
        })
    }

    pub async fn add_entries(&self, entries: impl Iterator<Item = Entry>) -> Result<(), Error> {
        let mut tx = self.pool.begin().await.map_err(Error::Transaction)?;
        for entry in entries {
            tx.execute(
                sqlx::query(&format!(
                    r#"
                    INSERT INTO {} (key, molecule, properties) VALUES (?, ?, ?)
                "#,
                    self.molecule_table
                ))
                .bind(entry.key)
                .bind(serde_json::to_string(&entry.molecule).unwrap())
                .bind(serde_json::to_string(&entry.properties).unwrap()),
            )
            .await
            .map_err(Error::Transaction)?;
        }
        tx.commit().await.map_err(Error::Transaction)?;
        Ok(())
    }
}

#[macro_export]
macro_rules! map {
    ($($args:tt)*) => {
        serde_json::from_value(serde_json::json!({ $($args)* })).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<(), Error> {
        let database = Database::connect("sqlite::memory:").await?;
        database
            .add_entries(
                [
                    Entry {
                        key: "first".into(),
                        molecule: Molecule {
                            atomic_numbers: vec![1, 2, 3],
                            atom_charges: Some(vec![0, 0, 0]),
                            bonds: Some(Bonds {
                                atom1: vec![0, 1],
                                atom2: vec![1, 2],
                                order: vec![1, 2],
                            }),
                            dative_bonds: None,
                            aromatic_bonds: None,
                            conformers: Some(vec![vec![
                                [0.0, 0.0, 0.0],
                                [1.0, 1.0, 1.0],
                                [2.0, 2.0, 2.0],
                            ]]),
                        },
                        properties: map! {
                            "density": 1.0,
                            "color": "colorless",
                        },
                    },
                    Entry {
                        key: "second".into(),
                        molecule: Molecule {
                            atomic_numbers: vec![4, 5],
                            atom_charges: None,
                            bonds: None,
                            dative_bonds: None,
                            aromatic_bonds: Some(AromaticBonds {
                                atom1: vec![0, 1],
                                atom2: vec![1, 2],
                            }),
                            conformers: Some(vec![vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]]]),
                        },
                        properties: map! {
                            "density": 2.0,
                            "odor": false,
                            "other": {
                                "optimized": true,
                            }
                        },
                    },
                ]
                .into_iter(),
            )
            .await?;
        Ok(())
    }
}
