/// Default role for users who are not logged in.
///
/// Represents Guest users or publicly accessible pages.
pub const PUBLIC: u32 = 0;

/// Anybody who is logged in is automatically assigned a user role.
pub const USER: u32 = 1;

/// Suggested role for flagging internal users
pub const INTERNAL: u32 = 2;

/// Suggested role for flagging managers
pub const MANAGER: u32 = 4;

/// Suggested role for flagging executives (CEO, VP, etc.)
pub const EXECUTIVE: u32 = 8;

/// MyFi integrated role for flagging Administrative users
pub const ADMIN: u32 = 536_870_912;

/// MyFi integrated role for flagging Site Administrative users (i.e. developers).
pub const SITE_ADMIN: u32 = 1_073_741_824;

/// Unassignable role used for setting up page folders that are hidden from displaying in the menu.
/// Pages within these folders would then be set with an accessible role.
/// The purpose of doing this is for page assignment of icons and title without displaying the page in the navigation menu.
pub const INVALID: u32 = 2_147_483_648;

pub fn roles_contains_role(roles: u32, role: u32) -> bool {
    roles & role == role
}

/// Helper method to translate a role level to a role value.
/// This is intended to help with creating additional or alternate roles to the ones provided by default in WebUI.
/// Role levels are 0 to 32.
///
/// Guest level is 0.
/// Example: role_from_level(0) returns PUBLIC
///
/// User level is 1.
/// Example: role_from_level(1) returns USER
///
/// Invalid level is 32.
/// Example: role_from_level(32) returns INVALID
pub fn role_from_level(level: u8) -> u32 {
    if level > 32 {
        return 0;
    }
    if level == 0 {
        return 0;
    }
    1 << (level - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_contains_roles() {
        assert!(roles_contains_role(USER | ADMIN, USER));
        assert!(!roles_contains_role(USER | ADMIN, SITE_ADMIN));
        assert!(!roles_contains_role(PUBLIC, SITE_ADMIN));
    }

    #[test]
    fn test_role_values() {
        assert_eq!(role_from_level(0), PUBLIC);
        assert_eq!(role_from_level(1), USER);
        assert_eq!(role_from_level(30), ADMIN);
        assert_eq!(role_from_level(31), SITE_ADMIN);
        assert_eq!(role_from_level(32), INVALID);
    }

    #[test]
    fn test_roles() {
        assert_eq!(PUBLIC, 0);
        assert_eq!(USER, 1);
        assert_eq!(ADMIN, 536_870_912);
        assert_eq!(SITE_ADMIN, 1_073_741_824);
        assert_eq!(PUBLIC, USER & ADMIN);
        assert_eq!(USER, USER & 3);
    }
}
