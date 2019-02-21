#[cfg(test)]
mod team_test
{
    use crate::app::team::Team;

    const TEAM_USER_NAME: &'static str = "RedTeam123";
    const TEAM_DISPLAY_NAME: &'static str = "Red Team";

    #[test]
    fn can_create_empty_team() {
        let team: Team = Team::new(TEAM_USER_NAME, TEAM_DISPLAY_NAME);

        assert_eq!(team.user_name, TEAM_USER_NAME);
        assert_eq!(team.display_name, TEAM_DISPLAY_NAME);
        assert_eq!(team.members.is_empty(), true);
    }
}