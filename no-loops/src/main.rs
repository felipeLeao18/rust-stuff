use std::{env, fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Produto {
    cod: u32,
    qtd: u32,
    valor_unitario: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Compra {
    data: String,
    produtos: Vec<Produto>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Raw {
    compras: Vec<Compra>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env::current_dir()?.join("src/file.json"));
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let raw: Raw = serde_json::from_str(&contents)?;
    let sum: f64 = raw
        .compras
        .iter()
        .flat_map(|c| {
            c.produtos
                .iter()
                .map(|p| -> f64 { Into::<f64>::into(p.qtd) * p.valor_unitario })
        })
        .sum();

    dbg!(sum);

    Ok(())
}
