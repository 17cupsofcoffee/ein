use std::cell::RefCell;
use std::rc::Rc;
use fnv::FnvHashMap;
use super::Value;

struct EnvInner {
    parent: Option<Env>,
    values: FnvHashMap<String, Value>,
}

#[derive(Clone)]
pub struct Env(Rc<RefCell<EnvInner>>);

impl Env {
    pub fn root() -> Env {
        let inner = EnvInner {
            parent: None,
            values: FnvHashMap::default(),
        };

        Env(Rc::new(RefCell::new(inner)))
    }

    pub fn child(parent: &Env) -> Env {
        let inner = EnvInner {
            parent: Some(parent.clone()),
            values: FnvHashMap::default(),
        };

        Env(Rc::new(RefCell::new(inner)))
    }

    pub fn declare(&mut self, name: String, value: Value) {
        self.0.borrow_mut().values.insert(name, value);
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
