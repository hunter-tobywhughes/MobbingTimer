use crate::app::user::User;

pub struct Team
{
    pub user_name: String,
    pub display_name: String,
    pub members: Vec<User>
}

impl Team
{
    pub fn new(user_name: &str, display_name: &str) -> Team
    {
        return Team {
            user_name: user_name.into(),
            display_name: display_name.into(),
            members: Vec::new()
        }
    }
}