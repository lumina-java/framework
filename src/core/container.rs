use std::collections::HashMap;
use std::any::Any;

pub struct Container {
    bindings: HashMap<String, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    
    pub fn bind<T: 'static>(&mut self, key: String, value: T) {
        self.bindings.insert(key, Box::new(value));
    }
}
