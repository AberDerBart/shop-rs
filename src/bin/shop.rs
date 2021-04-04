use anyhow::Result;
use shop_rs::{ops, Config};
use structopt::StructOpt;

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
    /// Remove an item from the list
    Remove {
        /// the string representation of an item,
        /// or the index of an item on the list
        item: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opt = Shop::from_args();
    println!("{:?}", opt);
    let config = gen_config(opt.server, opt.list, opt.proxy);

    match opt.cmd {
        Some(Command::Add { item: i }) => {
            println!("add {:#?}", i);
            let item = i.join(" ");
            let result = ops::add(&config, item);
            println!("add result {:#?}", result);
        }
        Some(Command::Remove { item: i }) => {
            println!("remove {:#?}", i);
            let item = i.join(" ");
            let result = match parse_index(&item) {
                Some(i) => ops::remove_by_index(&config, i),
                None => Ok(()), //remove_item(item),
            };
            println!("remove result {:#?}", result);
        }
        None => {
            let result = ops::print_list(&config);
            println!("remove result {:#?}", result);
        }
    }
    Ok(())
}

fn parse_index(i: &str) -> Option<usize> {
    i.parse::<usize>().ok()
}

fn gen_config(server: Option<String>, list: Option<String>, proxy: Option<String>) -> Config {
    let mut c = Config::default();
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
