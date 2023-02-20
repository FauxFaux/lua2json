mod parse;

use std::fs;
use std::io::{BufRead, Write};

use anyhow::{anyhow, Context, Result};
use serde_json as json;
use serde_json::json;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for (n, line) in stdin.lines().enumerate() {
        let line = line?;
        let obj =
            parse::parse(&line).with_context(|| anyhow!("parsing line {}: {line:?}", n + 1))?;
        serde_json::to_writer(&mut stdout, &to_json(&obj)?)?;
        stdout.write_all(b"\n")?;
    }
    Ok(())
}

fn to_json(table: &parse::Table) -> Result<json::Value> {
    let array = table.iter().all(|(k, _)| k.is_none());
    Ok(if array {
        json::Value::Array(
            table
                .into_iter()
                .map(|(_, v)| val_to_json(v))
                .collect::<Result<_>>()?,
        )
    } else {
        let mut obj = serde_json::Map::new();
        for (i, (k, v)) in table.into_iter().enumerate() {
            let k = k
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| i.to_string());
            obj.insert(k, val_to_json(v)?);
        }
        json::Value::Object(obj)
    })
}

fn val_to_json(v: &parse::Value) -> Result<json::Value> {
    Ok(match v {
        parse::Value::Float(f) => json!(*f),
        parse::Value::String(s) => json::Value::String(s.clone()),
        parse::Value::Object(t) => to_json(&t)?,
    })
}
