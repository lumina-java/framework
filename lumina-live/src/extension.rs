use lumina::prelude::Application;
use crate::component::LiveComponent;
use crate::registry::LiveRegistry;

pub trait LiveApplicationExt {
    /// Mengaktifkan runtime reaktif Lumina Live:
    /// - Mendaftarkan direktif Blade: `@kilat`, `@live`, `@livewire`, `@kilatScripts`, `@luminaLive`
    /// - Memasang rute transport WebSocket (`/_lumina/live/ws`), HTTP RPC (`/_lumina/live/call`), dan asset JS
    fn with_live(self) -> Self;

    /// Mendaftarkan komponen reaktif Lumina Live ke registry.
    fn with_live_component<T: LiveComponent>(self, name: &str) -> Self;
}

impl LiveApplicationExt for Application {
    fn with_live(self) -> Self {
        crate::init();
        self.with_raw_router(crate::routes())
    }

    fn with_live_component<T: LiveComponent>(self, name: &str) -> Self {
        crate::init();
        LiveRegistry::global().register::<T>(name);
        self
    }
}
