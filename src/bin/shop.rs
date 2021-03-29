use structopt::StructOpt;
use anyhow::Result;
use shop_rs::SLItem;


#[derive(StructOpt, Debug)]
#[structopt(about = "interact with shopping lists")]
enum Shop {
    Add {
        item: Vec<String>,
    },
    Remove {
        item: Vec<String>,
    },
}

fn main() -> Result<()> {
    let opt = Shop::from_args();
    println!("{:?}", opt);

    match opt {
        Shop::Add{item: i} => {
            let i = i.join(" ");
            println!("add {}", i);
        },
        Shop::Remove{item: i} => {
            let i = i.join(" ");
            println!("remove {}", i);
        },
    }
    Ok(())
}
