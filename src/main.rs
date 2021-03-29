use anyhow::Result;

use shop_rs::{Config, State, SyncRequest, SyncedShoppingList};

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

fn add(config: &Config, item: String) -> Result<()> {
    let agent = ureq::AgentBuilder::new()
        .proxy(ureq::Proxy::new("localhost:8080")?)
        .build();

    let mut state = get_current_list(&agent, config)?;
    state.current_state.add(item);
    sync(&agent, &config, state)?;

    Ok(())
}

fn main() -> Result<()> {
    let config = Config {
        server: "http://localhost:4000".to_owned(),
        list_id: "Demo".to_owned(),
    };

    add(&config, "(OG) foo".to_owned())
}
