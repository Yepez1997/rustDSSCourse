use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {

    pub fn get(&mut self, key: String) -> Option<String> {
        unimplemented!("unimplemented");
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!("unimplemented")
    }

    pub fn set(&mut self, key: String, value: String) {
        unimplemented!("unimplemented")
    }

    pub fn new() -> KvStore {
        KvStore {store: HashMap::new()}
    }
}


