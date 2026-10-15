<<<<<<< HEAD
pub mod google;
pub mod manager;
pub mod provider;

pub use google::GoogleProvider;
pub use manager::SocialiteManager;
pub use provider::{SocialProvider, SocialUser};
=======
pub mod provider;
pub mod manager;
pub mod google;

pub use provider::{SocialProvider, SocialUser};
pub use manager::SocialiteManager;
pub use google::GoogleProvider;
>>>>>>> dev/master
