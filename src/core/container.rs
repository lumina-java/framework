<<<<<<< HEAD
use std::any::Any;
use std::collections::HashMap;
=======
use std::collections::HashMap;
use std::any::Any;
>>>>>>> dev/master

pub struct Container {
    bindings: HashMap<String, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
<<<<<<< HEAD

=======

>>>>>>> dev/master
    pub fn bind<T: 'static>(&mut self, key: String, value: T) {
        self.bindings.insert(key, Box::new(value));
    }
}
