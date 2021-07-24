use anyhow::Result;
use shop_rs::{ops, Config};
use structopt::StructOpt;
#[macro_use]
extern crate log;

lazy_static::lazy_static! {
    static ref CONFIG: Config = {
        let foo = directories::ProjectDirs::from("","","shop-rs");
        match foo {
            Some(project_dir) => {
                let path = project_dir.config_dir().join("config.toml");
                match &std::fs::read(path) {
                    Ok(bytes) => toml::de::from_slice(bytes).unwrap_or_default(),
                    Err(_) => Default::default(),
                }},
            None => Default::default(),
        }
    };
}

#[derive(StructOpt, Debug)]
#[structopt(about = "interact with shopping lists")]
struct Shop {
    #[structopt(short, long)]
    /// server where the shopping list is managed
    server: Option<String>,
    #[structopt(short, long)]
    /// name of the shopping list
    list: Option<String>,
    #[structopt(short, long)]
    /// proxy server
    proxy: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Add an item to the list
    Add {
        /// the string representation of an item
        item: Vec<String>,
    },
    /// Delete an item from the list
    Del {
        /// the string representation of an item,
        /// or the index of an item on the list
        item: Vec<String>,
    },
    /// Edit a list item
    Edit {
        /// the index of the item to edit
        item: String,
        /// the string representation of the new item value
        value: Vec<String>,
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Shop::from_args();
    debug!("{:?}", opt);
    let config = gen_config(opt.server, opt.list, opt.proxy);

    match opt.cmd {
        Some(Command::Add { item: i }) => {
            debug!("add {:#?}", i);
            let item = i.join(" ");
            let result = ops::add(&config, item);
            debug!("add result {:#?}", result);
        }
        Some(Command::Del { item: i }) => {
            debug!("remove {:#?}", i);
            let item = i.join(" ");
            let result = match parse_index(&item) {
                Some(i) => ops::remove_by_index(&config, i),
                None => Ok(()), //remove_item(item),
            };
            debug!("remove result {:#?}", result);
        }
        Some(Command::Edit {item: i, value: v}) => {
            let v = v.join(" ");
            debug!("edit {:#?} to {:#?}",i,v);
            let result = match parse_index(&i) {
                Some(i) => ops::edit_by_index(&config, i, v),
                None => Ok(()),
            };
            debug!("edit result {:#?}", result);
        }
        None => {
            ops::print_list(&config)?;
        }
    }
    Ok(())
}

fn parse_index(i: &str) -> Option<usize> {
    i.parse::<usize>().ok().map(|i| i - 1)
}

fn gen_config(server: Option<String>, list: Option<String>, proxy: Option<String>) -> Config {
    let mut c = CONFIG.clone();
    if let Some(s) = server {
        c.server = s;
    };
    if let Some(l) = list {
        c.list_id = l
    }
    if let Some(p) = proxy {
        c.proxy = Some(p)
    }
    c
}
