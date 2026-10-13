pub struct User {
    pub name: String,
    pub username: String,
    password: String,
}

impl User {
    pub fn new(
        name: String, 
        username: String, 
        password: String
    ) -> Self {
        Self { name, username, password }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() 
        || self.username.is_empty()
        || self.password.is_empty()
    }
}
