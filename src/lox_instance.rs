#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub name: String,
    pub methods: i32,
}

impl LoxInstance {
    pub fn new(name: String, methods: i32) -> Self {
        Self { name, methods }
    }
    pub fn name(&self) -> String {
        format!("{} instance", self.name)
    }
}
