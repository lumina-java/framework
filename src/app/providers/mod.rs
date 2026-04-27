pub trait ServiceProvider {
    fn register(&self);
    fn boot(&self);
}
