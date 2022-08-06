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
    Categories {
        #[structopt(subcommand)]
        cat_cmd: Option<CategoryCommand>,
    },
    /// Saves the configuration (set by -s, -l, etc.)
    Save,
}

#[derive(StructOpt, Debug)]
enum CategoryCommand {
    // /// List categories
    // List,
    /// Add category
    Add {
        /// name of the category
        #[structopt(required(true))]
        name: Vec<String>,
        /// abbreviation of the category
        #[structopt(short, long)]
        short: Option<String>,
        /// color
        #[structopt(short, long)]
        color: Option<String>,
        /// enable light text (for other clients)
        #[structopt(long)]
        lighttext: bool,
    },
    /// Delete category
    Del {
        /// the abbreviation or index of the category to be deleted
        category: String,
    },
    Edit {
        /// the abbreviation or index of the category to be edited
        category: String,
        /// name
        #[structopt()]
        name: Option<String>,
        /// abbreviation of the category
        #[structopt(short, long)]
        short: Option<String>,
        /// color
        #[structopt(short, long)]
        color: Option<String>,
        /// enable light text (for other clients)
        #[structopt(long, conflicts_with = "no_lighttext")]
        lighttext: bool,
        /// disable light text (for other clients)
        #[structopt(long, conflicts_with = "lighttext")]
        no_lighttext: bool,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Shop::from_args();
    debug!("{:?}", opt);
    let config = gen_config(opt.server, opt.list, opt.proxy);

    match opt.cmd {
        Some(Command::Add { item }) => {
            debug!("add {:#?}", item);
            let config = &config;
            if item.len() > 0 {
                add(config, item.join(" "))
            } else {
                add_from_stdin(config)
            }
        }
        Some(Command::Del { item: i }) => {
            debug!("remove {:#?}", i);
            let item = i.join(" ");
            match parse_index(&item) {
                Some(i) => ops::remove_by_index(&config, i),
                None => Ok(()), //remove_item(item),
            }
        }
        Some(Command::Edit { item: i, value: v }) => {
            let v = v.join(" ");
            debug!("edit {:#?} to {:#?}", i, v);
            match parse_index(&i) {
                Some(i) => ops::edit_by_index(&config, i, v),
                None => Ok(()),
            }
        }
        Some(Command::Categories { cat_cmd: None }) => {
            debug!("categories list");
            ops::print_categories(&config)
        }
        Some(Command::Categories {
            cat_cmd:
                Some(CategoryCommand::Add {
                    name,
                    short,
                    color,
                    lighttext,
                }),
        }) => {
            debug!("categories add");
            ops::add_category(&config, name.join(" "), short, color, lighttext)
        }
        Some(Command::Categories {
            cat_cmd: Some(CategoryCommand::Del { category }),
        }) => {
            debug!("categories del");
            match parse_index(&category) {
                Some(index) => ops::remove_category_by_index(&config, index),
                None => Ok(()), // TODO: remove by abbreviation
            }
        }
        Some(Command::Categories {
            cat_cmd:
                Some(CategoryCommand::Edit {
                    name,
                    short,
                    category,
                    color,
                    lighttext,
                    no_lighttext,
                }),
        }) => {
            let lighttext_option = match (lighttext, no_lighttext) {
                (true, false) => Some(true),
                (false, true) => Some(false),
                _ => None,
            };
            match parse_index(&category) {
                Some(index) => ops::edit_category_by_index(
                    &config,
                    index,
                    name,
                    short,
                    color,
                    lighttext_option,
                ),
                None => Ok(()), // TODO: edit by abbreviation
            }
        }
        Some(Command::Save) => {
            debug!("save");
            write_config(&config)
        }
        None => ops::print_list(&config),
    }
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
