use structopt::StructOpt;
use anyhow::Result;
use shop_rs::{Config, opts};

#[derive(StructOpt, Debug)]
#[structopt(about = "interact with shopping lists")]
struct Shop {
    #[structopt(short, long)]
    /// server where the shopping list is managed
    server: Option<String>,
    #[structopt(short, long)]
    /// name of the shopping list
    list: Option<String>,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        /// the string representation of an item
        item: Vec<String>,
    },
    Remove {
        /// the string representation of an item,
        /// or the index of an item on the list
        item: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opt = Shop::from_args();
    println!("{:?}", opt);

    match opt.cmd {
        Command::Add{item: i} => {
            println!("add {:#?}", i);
            let config = gen_config(opt.server, opt.list);
            let item = i.join(" ");
            let result = opts::add(&config, item);
            println!("add result {:#?}", result);
        },
        Command::Remove{item: i} => {
            println!("remove {:#?}", i);
            let config = gen_config(opt.server, opt.list);
            let item = i.join(" ");
            let result = match parse_index(&item) {
                Some(i) => opts::remove_by_index(&config, i),
                None => Ok(()),//remove_item(item),
            };
            println!("remove result {:#?}", result);
        },
    }
    Ok(())
}

fn parse_index(i: &str) -> Option<usize>{
    i.parse::<usize>().ok()
}

fn gen_config(server: Option<String>, list: Option<String>) -> Config {
    let mut c = Config::default();
    if let Some(s) = server {
        c.server = s;
    };
    if let Some(l) = list {
        c.list_id = l
    }
    c
}
