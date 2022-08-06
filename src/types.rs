use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

extern crate colored;
use colored::*;

use css_color_parser2::Color;

#[derive(PartialEq)]
pub enum IncludeCategories {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amount {
    value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:.2}", self.value)?;
        if let Some(u) = &self.unit {
            write!(f, " {}", u)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryDefinition {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub color: String,
    #[serde(rename = "lightText")]
    pub light_text: bool,
}

impl Display for CategoryDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.color.parse::<Color>();
        let colorblock = color
            .map(|c| " ".on_truecolor(c.r, c.g, c.b))
            .unwrap_or(" ".normal());
        write!(f, "{}({})", colorblock, self.short_name)?;
        Ok(())
    }
}

impl CategoryDefinition {
    pub fn to_string_long(&self) -> String {
        let color = self.color.parse::<Color>();
        let colorblock = color
            .map(|c| " ".on_truecolor(c.r, c.g, c.b))
            .unwrap_or(" ".normal());
        format!("{}({}) {}", colorblock, self.short_name, self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SLItem {
    ServerRepr {
        id: Uuid,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        category: Option<Uuid>,
    },
    StringRepr {
        id: Uuid,
        #[serde(rename = "stringRepresentation")]
        string_representation: String,
    },
}

impl Display for SLItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SLItem::ServerRepr {
                id: _,
                name,
                amount,
                category: _,
            } => {
                if let Some(a) = amount {
                    write!(f, "{} ", a)?;
                }
                write!(f, "{}", name)?;
            }
            SLItem::StringRepr {
                id: _,
                string_representation,
            } => {
                write!(f, "{}", string_representation)?;
            }
        }
        Ok(())
    }
}

impl SLItem {
    pub fn new(string_representation: String) -> Self {
        SLItem::StringRepr {
            id: Uuid::new_v4(),
            string_representation,
        }
    }

    pub fn id(&self) -> &Uuid {
        match self {
            SLItem::ServerRepr {
                id,
                name: _,
                amount: _,
                category: _,
            } => id,
            SLItem::StringRepr {
                id,
                string_representation: _,
            } => id,
        }
    }

    pub fn edit(&mut self, string_representation: String) {
        let id = match self {
            SLItem::ServerRepr {
                id,
                name: _,
                amount: _,
                category: _,
            } => *id,
            SLItem::StringRepr {
                id,
                string_representation: _,
            } => *id,
        };
        *self = SLItem::StringRepr {
            id,
            string_representation,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    id: String,
    title: String,
    pub items: Vec<SLItem>,
}

impl ShoppingList {
    pub fn add(&mut self, string_representation: String) {
        self.items.push(SLItem::new(string_representation));
    }

    pub fn remove_by_index(&mut self, index: usize) -> Result<()> {
        if index < self.items.len() {
            self.items.remove(index);
            Ok(())
        } else {
            Err(anyhow!("invalid index"))
        }
    }

    pub fn edit_by_index(&mut self, index: usize, new_value: String) -> Result<()> {
        match self.items.get_mut(index) {
            None => Err(anyhow!("invalid index")),
            Some(item) => Ok(item.edit(new_value)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncedShoppingList {
    #[serde(flatten)]
    pub list: ShoppingList,
    token: String,
    #[serde(rename = "changeId")]
    change_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SyncResponse {
    List(SyncedShoppingList),
    Response {
        list: SyncedShoppingList,
        categories: Vec<CategoryDefinition>,
    },
}

#[derive(Serialize, Debug, Clone)]
pub struct SyncRequest {
    #[serde(rename = "previousSync")]
    previous_sync: SyncedShoppingList,
    #[serde(rename = "currentState")]
    current_state: ShoppingList,
    #[serde(rename = "includeInResponse")]
    include_in_reponse: Vec<String>,
    categories: Option<Vec<CategoryDefinition>>,
    // TODO: orders
    // TODO: deleteCompletions
    // TODO: addCompletions
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    #[serde(rename = "previousSync")]
    previous_sync: SyncedShoppingList,
    #[serde(rename = "currentState")]
    pub current_state: ShoppingList,
    pub categories: Vec<CategoryDefinition>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", &self.current_state.title)?;
        let num_digits = format!("{}", self.current_state.items.len()).len();
        for (index, item) in self.current_state.items.iter().enumerate() {
            let category = if let SLItem::ServerRepr {
                id: _,
                name: _,
                category,
                amount: _,
            } = item
            {
                self.categories.iter().find(|c| &Some(c.id) == category)
            } else {
                None
            };
            match category {
                Some(category) => {
                    writeln!(f, "{:>n$}.{} {}", index + 1, category, item, n = num_digits)?
                }
                None => writeln!(f, "{:>n$}. {}", index + 1, item, n = num_digits)?,
            }
        }
        Ok(())
    }
}

impl From<SyncResponse> for State {
    fn from(sr: SyncResponse) -> Self {
        match sr {
            SyncResponse::List(list) => State {
                current_state: list.list.clone(),
                previous_sync: list,
                categories: vec![],
            },
            SyncResponse::Response { list, categories } => State {
                current_state: list.list.clone(),
                previous_sync: list,
                categories: categories,
            },
        }
    }
}

impl SyncRequest {
    pub fn new(previous_sync: SyncedShoppingList, current_state: ShoppingList) -> Self {
        SyncRequest {
            previous_sync,
            current_state,
            include_in_reponse: vec![],
            categories: None,
        }
    }

    pub fn from_state(state: State, include_categories: IncludeCategories) -> Self {
        SyncRequest {
            previous_sync: state.previous_sync,
            current_state: state.current_state,
            include_in_reponse: vec!["categories".to_owned()],
            categories: if include_categories == IncludeCategories::Yes {
                Some(state.categories)
            } else {
                None
            },
        }
    }
}
