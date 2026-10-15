pub mod event_service_provider;

pub trait ServiceProvider {

    fn register(&self);
    fn boot(&self);
}
