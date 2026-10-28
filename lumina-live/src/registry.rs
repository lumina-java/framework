use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use lumina::core::error::AppError;
use lumina::core::blade::BladeEngine;
use crate::component::LiveComponent;

#[async_trait]
pub trait LiveComponentHandler: Send + Sync {
    async fn call(
        &self,
        state: Value,
        action: &str,
        params: Vec<Value>,
        engine: &BladeEngine,
    ) -> Result<(Value, String), AppError>;

    fn render_initial(
        &self,
        initial_state: Value,
        engine: &BladeEngine,
    ) -> Result<(Value, String), AppError>;
}

pub struct ComponentHandler<T: LiveComponent> {
    _marker: PhantomData<T>,
}

impl<T: LiveComponent> Default for ComponentHandler<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T: LiveComponent> LiveComponentHandler for ComponentHandler<T> {
    async fn call(
        &self,
        state: Value,
        action: &str,
        params: Vec<Value>,
        engine: &BladeEngine,
    ) -> Result<(Value, String), AppError> {
        let mut comp: T = serde_json::from_value(state)
            .map_err(|e| AppError::BadRequest(format!("Gagal hidrasi state komponen: {}", e)))?;

        comp.call_action(action, params).await?;

        let new_state = serde_json::to_value(&comp)
            .map_err(|e| AppError::Generic(format!("Gagal serialisasi state komponen: {}", e)))?;

        let html = comp.render_html(engine)?;
        Ok((new_state, html))
    }

    fn render_initial(
        &self,
        initial_state: Value,
        engine: &BladeEngine,
    ) -> Result<(Value, String), AppError> {
        let comp: T = if initial_state.is_null() || initial_state.as_object().map_or(false, |m| m.is_empty()) {
            T::default()
        } else {
            let mut val = serde_json::to_value(T::default())
                .unwrap_or(serde_json::json!({}));
            if let (Some(base_obj), Some(init_obj)) = (val.as_object_mut(), initial_state.as_object()) {
                for (k, v) in init_obj {
                    base_obj.insert(k.clone(), v.clone());
                }
            }
            serde_json::from_value(val)
                .map_err(|e| AppError::BadRequest(format!("Format data awal komponen salah: {}", e)))?
        };

        let state = serde_json::to_value(&comp)
            .map_err(|e| AppError::Generic(format!("Gagal serialisasi state komponen: {}", e)))?;

        let html = comp.render_html(engine)?;
        Ok((state, html))
    }
}

#[derive(Clone, Default)]
pub struct LiveRegistry {
    handlers: Arc<RwLock<HashMap<String, Arc<dyn LiveComponentHandler>>>>,
}

impl LiveRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn global() -> &'static LiveRegistry {
        static INSTANCE: std::sync::OnceLock<LiveRegistry> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(LiveRegistry::new)
    }

    pub fn register<T: LiveComponent>(&self, name: &str) {
        let handler = Arc::new(ComponentHandler::<T>::default());
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(name.to_string(), handler.clone());

        let global_handlers = &Self::global().handlers;
        if !Arc::ptr_eq(&self.handlers, global_handlers) {
            let mut g = global_handlers.write().unwrap();
            g.insert(name.to_string(), handler);
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn LiveComponentHandler>> {
        let handlers = self.handlers.read().unwrap();
        handlers.get(name).cloned()
    }

    pub fn has(&self, name: &str) -> bool {
        let handlers = self.handlers.read().unwrap();
        handlers.contains_key(name)
    }
}
