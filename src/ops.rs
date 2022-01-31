use super::{Config, State, SyncRequest};
use crate::{CategoryDefinition, SyncResponse};
use std::io::{self, BufRead};
use anyhow::{anyhow, Result};
use uuid::Uuid;

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

fn sync(
    agent: &ureq::Agent,
    config: &Config,
    state: State,
    include_categories: bool,
) -> Result<State> {
    let mut path = config.path();
    path.push_str("/sync");

    let data = SyncRequest::from_state(state, include_categories);

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

    let state = sync(&agent, &config, state, false)?;

    print!("{}", state);

    Ok(())
}

pub fn add(config: &Config, item: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.add(item);
    let state = sync(&agent, &config, state, false)?;

    print!("{}", state);

    Ok(())
}

pub fn edit_by_index(config: &Config, index: usize, value: String) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.edit_by_index(index, value)?;
    let state = sync(&agent, &config, state, false)?;

    print!("{}", state);

    Ok(())
}

pub fn remove_by_index(config: &Config, index: usize) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    state.current_state.remove_by_index(index)?;
    let state = sync(&agent, &config, state, false)?;

    print!("{}", state);

    Ok(())
}

pub fn print_list(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let state = get_current_list(&agent, config)?;

    print!("{}", state);

    Ok(())
}

fn print_categories_internal(categories: &Vec<CategoryDefinition>) {
    let num_digits = format!("{}", categories.len()).len();
    for (index, cat) in categories.iter().enumerate() {
        println!("{:>n$}.{}", index + 1, cat.to_string_long(), n = num_digits);
    }
}

pub fn print_categories(config: &Config) -> Result<()> {
    let agent = get_agent(config)?;

    let state = get_current_list(&agent, config)?;

    print_categories_internal(&state.categories);
    Ok(())
}

fn random_color() -> String {
    let r = rand::random::<u8>();
    let g = rand::random::<u8>();
    let b = rand::random::<u8>();

    format!("#{:0>2x}{:0>2x}{:0>2x}", r, g, b)
}

fn derive_category_short_name(name: &str) -> String {
    let short: String = name.chars().filter(|c| c.is_uppercase()).collect();

    if short.len() > 0 {
        return short;
    }

    if name.len() >= 3 {
        return name[..3].to_uppercase();
    }

    name.to_uppercase()
}

pub fn add_category(
    config: &Config,
    name: String,
    short_name: Option<String>,
    color: Option<String>,
    light_text: bool,
) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    // TODO: make sure state is synced

    let short_name = short_name.unwrap_or_else(|| derive_category_short_name(&name));
    let color = color.unwrap_or_else(|| random_color());

    state.categories.push(CategoryDefinition {
        name,
        color,
        short_name,
        light_text,
        id: Uuid::new_v4(),
    });

    let state = sync(&agent, &config, state, true)?;

    print_categories_internal(&state.categories);

    Ok(())
}

pub fn remove_category_by_index(config: &Config, index: usize) -> Result<()> {
    let agent = get_agent(config)?;

    let mut state = get_current_list(&agent, config)?;
    // TODO: make sure state is synced

    if index >= state.categories.len() {
        return Err(anyhow!("invalid index"));
    }

    state.categories.remove(index);
    let state = sync(&agent, &config, state, true)?;

    print_categories_internal(&state.categories);

    Ok(())
}
