use anyhow::Result;

use shop_rs::{opts, Config};

fn main() -> Result<()> {
    let config = Config {
        server: "http://localhost:4000".to_owned(),
        list_id: "Demo".to_owned(),
    };

    opts::add(&config, "(OG) foo".to_owned())
}
