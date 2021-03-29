use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Amount {
    value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
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

    pub fn remove(&mut self, item: &SLItem) {
        self.items.retain(|i| i.id() != item.id());
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
pub struct SyncRequest {
    #[serde(rename = "previousSync")]
    previous_sync: SyncedShoppingList,
    #[serde(rename = "currentState")]
    current_state: ShoppingList,
    #[serde(rename = "includeInResponse")]
    include_in_reponse: Vec<String>,
    // TODO: categories
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
}

impl State {
    pub fn new(previous_sync: SyncedShoppingList) -> Self {
        let current_state = previous_sync.list.clone();
        State {
            previous_sync,
            current_state,
        }
    }
}

impl SyncRequest {
    pub fn new(previous_sync: SyncedShoppingList, current_state: ShoppingList) -> Self {
        SyncRequest {
            previous_sync,
            current_state,
            include_in_reponse: vec![],
        }
    }
}

impl From<State> for SyncRequest {
    fn from(state: State) -> Self {
        SyncRequest {
            previous_sync: state.previous_sync,
            current_state: state.current_state,
            include_in_reponse: vec![],
        }
    }
}
