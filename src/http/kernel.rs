pub struct HttpKernel;

impl HttpKernel {
    pub fn new() -> Self {
        Self
    }
    
    pub fn bootstrap(&self) {
        println!("⚙️  HTTP Kernel bootstrapped");
    }
}
