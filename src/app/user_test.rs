#[cfg(test)]
mod user_test
{
    use crate::app::user::User;

    const USER_USER_NAME: &'static str = "Bob123";
    const USER_DISPLAY_NAME: &'static str = "Bob";

    #[test]
    fn can_create_user() {
        let user: User = User::new(USER_USER_NAME, USER_DISPLAY_NAME);
        assert_eq!(user.user_name, USER_USER_NAME);
        assert_eq!(user.display_name, USER_DISPLAY_NAME);
    }
}