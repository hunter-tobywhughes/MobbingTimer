pub struct User {
    pub user_name: String,
    pub display_name: String,
}

impl User {
    pub fn new(user_name: &str, display_name: &str) -> User {
        return User {
            user_name: user_name.into(),
            display_name: display_name.into(),
        };
    }
}
