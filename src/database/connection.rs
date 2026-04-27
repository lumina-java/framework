#[allow(dead_code)]
pub struct Database {
    connection_string: String,
}

impl Database {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
    
    pub async fn connect(&self) -> Result<(), String> {
        println!("📦 Database connected");
        Ok(())
    }
}
