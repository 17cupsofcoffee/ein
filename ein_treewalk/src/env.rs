use super::Value;
use fnv::FnvHashMap;
use std::cell::RefCell;
use std::rc::Rc;

// TODO: This implementation of environments causes reference cycles (and in turn, memory leaks).
// To fix this, I'll need to either make the `parent` field use weak pointers or switch to a
// different structure (e.g. referring to environments by an id/index).
//
// See: https://doc.rust-lang.org/book/second-edition/ch15-06-reference-cycles.html

#[derive(Debug, PartialEq)]
struct EnvInner {
    parent: Option<Env>,
    values: FnvHashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env(Rc<RefCell<EnvInner>>);

impl Env {
    pub fn root() -> Env {
        let inner = EnvInner {
            parent: None,
            values: FnvHashMap::default(),
        };

        Env(Rc::new(RefCell::new(inner)))
    }

    pub fn child(&self) -> Env {
        let inner = EnvInner {
            parent: Some(self.clone()),
            values: FnvHashMap::default(),
        };

        Env(Rc::new(RefCell::new(inner)))
    }

    pub fn declare(&mut self, name: String, value: Value) {
        self.0.borrow_mut().values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        let mut inner = self.0.borrow_mut();
        if inner.values.contains_key(&name) {
            inner.values.insert(name, value);
            Ok(())
        } else {
            match inner.parent {
                Some(ref mut parent) => parent.assign(name, value),
                None => Err(format!("Variable {} has not been defined", name)),
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        // TODO: Can we do this without cloning? Do we even need to?
        // Once we're interning strings and storing references to objects,
        // Value can probably just impl Copy
        let inner = self.0.borrow();
        match inner.values.get(name) {
            Some(value) => Some(value.clone()),
            None => match inner.parent {
                Some(ref parent) => parent.get(name),
                None => None,
            },
        }
    }
}
