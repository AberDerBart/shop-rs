use std::path::PathBuf;

use anyhow::{anyhow, Result};
use shop_rs::{
    ops::{self, add, add_from_stdin},
    Config,
};
use structopt::StructOpt;
#[macro_use]
extern crate log;

fn config_path() -> Result<PathBuf> {
    let project_dir = directories::ProjectDirs::from("", "", "shop-rs")
        .ok_or(anyhow!("Could not find config directory"))?;
    let path = project_dir.config_dir().join("config.toml");

    Ok(path)
}

lazy_static::lazy_static! {
    static ref CONFIG: Config = {
        match config_path() {
            Ok(path) => {
                match &std::fs::read(path) {
                    Ok(bytes) => toml::de::from_slice(bytes).unwrap_or_default(),
                    Err(_) => Default::default(),
                }},
            _ => {Default::default()},
        }
    };
}

fn write_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    let content = toml::ser::to_string(config)?;

    debug!("writing config to {:?}", path);
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(path, content)?;

    Ok(())
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
    },
    /// List categories
    Categories,
    /// Saves the configuration (set by -s, -l, etc.)
    Save,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Shop::from_args();
    debug!("{:?}", opt);
    let config = gen_config(opt.server, opt.list, opt.proxy);

    match opt.cmd {
        Some(Command::Add { item }) => {
            debug!("add {:#?}", item);
            let result = {
                let config = &config;
                if item.len() > 0 {
                    add(config, item.join(" "))
                } else {
                    add_from_stdin(config)
                }
            };
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
        Some(Command::Edit { item: i, value: v }) => {
            let v = v.join(" ");
            debug!("edit {:#?} to {:#?}", i, v);
            let result = match parse_index(&i) {
                Some(i) => ops::edit_by_index(&config, i, v),
                None => Ok(()),
            };
            debug!("edit result {:#?}", result);
        }
        Some(Command::Categories) => {
            debug!("categories");
            let result = ops::print_categories(&config)?;
            debug!("categories result {:#?}", result);
        }
        Some(Command::Save) => {
            debug!("save");
            write_config(&config)?;
            debug!("config written");
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
