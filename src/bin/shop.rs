use structopt::StructOpt;
use anyhow::Result;

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
        /// the string representation of an item
        item: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opt = Shop::from_args();
    println!("{:?}", opt);

    match opt.cmd {
        Command::Add{item: i} => {
            let i = i.join(" ");
            println!("add {}", i);
        },
        Command::Remove{item: i} => {
            let i = i.join(" ");
            println!("remove {}", i);
        },
    }
    Ok(())
}
