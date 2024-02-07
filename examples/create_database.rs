use atomlite::{map, Database, Entry, Error, Molecule};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let path = std::env::args().nth(1).unwrap();
    let database = Database::connect(&format!("sqlite:{}", path)).await?;
    database
        .add_entries(
            [
                Entry {
                    key: "first".into(),
                    molecule: Molecule {
                        atomic_numbers: vec![1, 2, 3],
                        atom_charges: Some(vec![0, 0, 0]),
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
