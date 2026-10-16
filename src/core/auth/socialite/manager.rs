use crate::core::auth::socialite::provider::SocialProvider;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SocialiteManager {
    providers: HashMap<String, Arc<dyn SocialProvider>>,
}

impl SocialiteManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Arc<dyn SocialProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    pub fn driver(&self, name: &str) -> Option<Arc<dyn SocialProvider>> {
        self.providers.get(name).cloned()
    }
}
