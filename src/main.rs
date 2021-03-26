use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Amount {
    value: f64,
    unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SLItemServer {
    id: String,
    name: String,
    amount: Option<Amount>,
    category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SLItemString {
    id: Option<String>,
    #[serde(rename = "stringRepresentation")]
    string_representation: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum SLItem {
    Server {
        id: String,
        name: String,
        amount: Option<Amount>,
        category: Option<String>,
    },
    StringRepr {
        // #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(rename = "stringRepresentation")]
        string_representation: String,
    },
}

impl SLItem {
    fn set_name(&mut self, new_name: &str) {
        match self {
            SLItem::Server {
                id: _,
                name,
                amount: _,
                category: _,
            } => {
                *name = new_name.to_owned();
            }
            SLItem::StringRepr {
                id: _,
                string_representation,
            } => *string_representation = new_name.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ShoppingList {
    id: String,
    title: String,
    items: Vec<SLItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedShoppingList {
    id: String,
    title: String,
    items: Vec<SLItem>,
    token: String,
    #[serde(rename = "changeId")]
    change_id: String,
}

fn main() -> Result<()> {
    let body: String = ureq::get("http://192.168.178.184:4000/api/Demo")
        .call()?
        .into_string()?;

    let mut list: ShoppingList = serde_json::from_str(&body)?;
    // list.add(SLItem::StringRepr {
    //     id: None,
    //     string_representation: "bar".to_owned(),
    // });

    list.title = "foobazasdads".into();

    println!("{:#?}", list);

    let ser_list = serde_json::to_string(&list)?;

    println!("{}", ser_list);

    let val = serde_json::to_value(&list)?;
    println!("PUT Request: {:#?}", val);

    let resp: std::result::Result<ureq::Response, ureq::Error> =
        ureq::put("http://192.168.178.184:4000/api/Demo")
            // .send_string(&val);
            .send_json(val);
    match resp {
        Err(e) => match e {
            ureq::Error::Status(_, r) => {
                println!("{:?}", r.into_string()?);
            }
            ureq::Error::Transport(_) => {}
        },
        Ok(r) => {
            println!("{:?}", r.into_string()?);
        }
    }
    // .into_string();
    // let resp: ShoppingList = serde_json::from_str(&resp)?;

    // println!("Response: {:#?}", resp);
    Ok(())
}
