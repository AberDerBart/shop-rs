use super::{Config, State, SyncRequest};
use crate::SyncResponse;
use anyhow::Result;
use std::io::{self, BufRead};

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

    let data: SyncRequest = state.into();

    let mut req = agent.post(&path);
    if let Some(ref u) = config.username {
        req = req.set("X-ShoppingList-Username", u)
    };
    let resp = req.send_json(serde_json::to_value(data)?)?;
    let resp: SyncResponse = resp.into_json()?;

    Ok(resp.into())
}

fn get_current_list(agent: &ureq::Agent, config: &Config) -> Result<State> {
    let state = initial_sync(agent, config)?;

    Ok(state)
}

pub fn add_from_stdin(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line?;
        if !line.is_empty() {
            state.current_state.add(line);
        }
    }

    let state = sync(&agent, &config, state)?;

    print!("{}", state);

    Ok(())
}

pub fn add(config: &Config, item: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.add(item);
    let state = sync(&agent, &config, state)?;

    print!("{}", state);

    Ok(())
}

pub fn edit_by_index(config: &Config, index: usize, value: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.edit_by_index(index, value)?;
    let state = sync(&agent, &config, state)?;

    print!("{}", state);

    Ok(())
}

pub fn remove_by_index(config: &Config, index: usize) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.remove_by_index(index)?;
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

pub fn print_categories(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let state = get_current_list(&agent, config)?;
    for cat in &state.categories {
        cat.println_long();
    }

    Ok(())
}
