use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Amount {
    value: f64,
    unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SLItem {
    id: String,
    name: String,
    amount: Option<Amount>,
    category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShoppingList {
    id: String,
    title: String,
    items: Vec<SLItem>,
}

fn main() -> Result<()> {
    let body: String = ureq::get("http://list.tilman.ninja/api/Demo")
        .call()?
        .into_string()?;

    let list: ShoppingList = serde_json::from_str(&body)?;
    println!("{:#?}", list);
    Ok(())
}
