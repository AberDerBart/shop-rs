use super::{Config, State, SyncRequest, SyncedShoppingList};
use anyhow::Result;

fn get_agent(config: &Config) -> Result<ureq::Agent> {
    let agent = match &config.proxy {
        Some(proxy) => ureq::AgentBuilder::new()
            .proxy(ureq::Proxy::new(proxy)?)
            .build(),
        None => ureq::AgentBuilder::new().build(),
    };
    Ok(agent)
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

pub fn add(config: &Config, item: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.add(item);
    let state = sync(&agent, &config, state)?;

    print!("{}", state.current_state);

    Ok(())
}

pub fn remove_by_index(config: &Config, index: usize) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.remove_by_index(index);
    let state = sync(&agent, &config, state)?;

    print!("{}", state.current_state);

    Ok(())
}

pub fn print_list(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let state = get_current_list(&agent, config)?;

    print!("{}", state.current_state);

    Ok(())
}
