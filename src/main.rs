use anyhow::Result;

use shop_rs::{SLItem, State, SyncRequest, SyncedShoppingList};

struct Config {
    server: String,
    list_id: String,
}

impl Config {
    fn path(&self) -> String {
        format!("{}/api/{}", self.server, self.list_id)
    }
}

fn initial_sync(agent: &ureq::Agent, config: &Config) -> Result<SyncedShoppingList> {
    let mut path = config.path();
    path.push_str("/sync");
    let resp: ureq::Response = agent.get(&path).call()?;

    let resp: SyncedShoppingList = resp.into_json()?;
    Ok(resp)
}

fn sync(agent: &ureq::Agent, config: &Config, state: State) -> Result<State> {
    let mut path = config.path();
    path.push_str("/sync");

    let req: SyncRequest = state.into();

    let resp: ureq::Response = agent.post(&path).send_json(serde_json::to_value(req)?)?;
    let resp: SyncedShoppingList = resp.into_json()?;
    Ok(State::new(resp))
}

fn get_current_list(agent: &ureq::Agent, config: &Config) -> Result<State> {
    let synced_list = initial_sync(agent, config)?;

    Ok(State::new(synced_list))
}

fn main() -> Result<()> {
    let config = Config {
        server: "http://localhost:4000".to_owned(),
        list_id: "Demo".to_owned(),
    };

    let agent = ureq::AgentBuilder::new()
        .proxy(ureq::Proxy::new("localhost:8080")?)
        .build();

    let mut state = get_current_list(&agent, &config)?;
    println!("initial state: {:#?}", state);

    let item = SLItem::new("(OG) test".to_owned());
    state.current_state.add(item);

    let state = sync(&agent, &config, state);

    println!("{:#?}", state);

    Ok(())
}
