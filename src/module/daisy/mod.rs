use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

pub mod api;
pub mod dao;
pub mod bs;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Daisy {
    id: i64,
    name: String,
    value: u64,
    parent: i64,
    level: Option<isize>,
    children: Option<Vec<Rc<RefCell<Daisy>>>>,
}

impl Daisy {
    pub fn set_id(&mut self, id: i64) {
        self.id = id
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn set_children(mut self, children: Option<Vec<Rc<RefCell<Daisy>>>>) -> Self {
        self.children = children;
        self
    }
}
