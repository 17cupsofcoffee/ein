use super::value::Value;
use fnv::FnvHashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Env {
    parent: Weak<RefCell<Env>>,
    values: FnvHashMap<String, Value>,
}

pub type EnvRef = Rc<RefCell<Env>>;

impl Env {
    pub fn root() -> EnvRef {
        let inner = Env {
            parent: Weak::new(),
            values: FnvHashMap::default(),
        };

        Rc::new(RefCell::new(inner))
    }

    pub fn child(parent: &EnvRef) -> EnvRef {
        let inner = Env {
            parent: Rc::downgrade(parent),
            values: FnvHashMap::default(),
        };

        Rc::new(RefCell::new(inner))
    }

    pub fn declare(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            match self.parent.upgrade() {
                Some(parent) => parent.borrow_mut().assign(name, value),
                None => Err(format!("Variable {} has not been defined", name)),
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        // TODO: Can we do this without cloning? Do we even need to?
        // Once we're interning strings and storing references to objects,
        // Value can probably just impl Copy
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match self.parent.upgrade() {
                Some(parent) => parent.borrow().get(name),
                None => None,
            },
        }
    }
}
