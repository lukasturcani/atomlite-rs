use atomlite::{map, AromaticBonds, Bonds, Database, Entry, Error, Molecule};

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
                            atom1: vec![0],
                            atom2: vec![1],
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
