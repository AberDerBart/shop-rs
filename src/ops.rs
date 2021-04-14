use super::{Config, State, SyncRequest};
use crate::SyncResponse;
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

fn initial_sync(agent: &ureq::Agent, config: &Config) -> Result<State> {
    let mut path = config.path();
    path.push_str("/sync");
    let resp = agent
        .get(&path)
        .query("includeInResponse", "categories")
        .call()?;

    let resp: SyncResponse = resp.into_json()?;

    Ok(resp.into())
}

fn sync(agent: &ureq::Agent, config: &Config, state: State) -> Result<State> {
    let mut path = config.path();
    path.push_str("/sync");

    let req: SyncRequest = state.into();

    let resp: ureq::Response = agent.post(&path).send_json(serde_json::to_value(req)?)?;
    let resp: SyncResponse = resp.into_json()?;

    Ok(resp.into())
}

fn get_current_list(agent: &ureq::Agent, config: &Config) -> Result<State> {
    let state = initial_sync(agent, config)?;

    Ok(state)
}

pub fn add(config: &Config, item: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.add(item);
    let state = sync(&agent, &config, state)?;

    print!("{}", state);

    Ok(())
}

pub fn remove_by_index(config: &Config, index: usize) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.remove_by_index(index);
    let state = sync(&agent, &config, state)?;

    print!("{}", state);

    Ok(())
}

pub fn print_list(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let state = get_current_list(&agent, config)?;

    print!("{}", state);

    Ok(())
}
