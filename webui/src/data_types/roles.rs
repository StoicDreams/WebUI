/// Default role for users who are not logged in.
///
/// Represents Guest users or publicly accessible pages.
pub const PUBLIC: u64 = 0;

/// Anybody who is logged in is automatically assigned a user role.
pub const USER: u64 = 1;

pub const CLIENT: u64 = 2;

pub const EDITOR: u64 = 8;

pub const CONTRIBUTER: u64 = 16;

pub const VIEWER: u64 = 32;

pub const BILLING: u64 = 64;

pub const OWNER: u64 = 128;

pub const OPERATIONS: u64 = 256;

pub const IT: u64 = 512;

pub const MEMBER: u64 = 1024;

pub const EXTERNAL: u64 = 2048;

pub const INTERNAL: u64 = 4096;

pub const EXECUTIVE: u64 = 8192;

pub const MANAGER: u64 = 16384;

pub const PRODUCTION: u64 = 32768;

pub const CEO: u64 = 65536;

pub const COO: u64 = 131072;

pub const CTO: u64 = 262144;

pub const CFO: u64 = 524288;

pub const CMO: u64 = 1048576;

pub const PRESIDENT: u64 = 2097152;

pub const VICEPRESIDENT: u64 = 4194304;

pub const ASSISTANT: u64 = 8388608;

pub const MARKETING: u64 = 16777216;

pub const PRODUCT: u64 = 33554432;

pub const FINANCE: u64 = 67108864;

pub const HUMANRESOURCES: u64 = 134217728;

pub const BUSINESS: u64 = 268435456;

pub const PERSONNEL: u64 = 536870912;

pub const CUSTOMERSERVICE: u64 = 1073741824;

pub const ADMINISTRATIVE: u64 = 2147483648;

pub const ADMIN: u64 = 2_305_843_009_213_693_952;

pub const SITE_ADMIN: u64 = 4_611_686_018_427_387_904;

/// Role used for setting up page folders that are hidden from displaying in the menu.
/// Pages within these folders would then be set with an accessible role.
/// The purpose of doing this is for page assignment of icons and title without displaying the page in the navigation menu.
pub const INVALID: u64 = 9_223_372_036_854_775_808;

pub fn roles_contains_role(roles: u64, role: u64) -> bool {
    roles & role == role
}

/// Helper method to translate a role level to a role value.
/// This is intended to help with creating additional or alternate roles to the ones provided by default in WebUI.
/// Role levels are 0 to 64.
///
/// Guest level is 0.
/// Example: role_from_level(0) returns PUBLIC
///
/// User level is 1.
/// Example: role_from_level(1) returns USER
///
/// Invalid level is 64.
/// Example: role_from_level(64) returns INVALID
pub fn role_from_level(level: u8) -> u64 {
    if level > 64 {
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
        assert_eq!(role_from_level(62), ADMIN);
        assert_eq!(role_from_level(63), SITE_ADMIN);
        assert_eq!(role_from_level(64), INVALID);
    }

    #[test]
    fn test_roles() {
        assert_eq!(PUBLIC, 0);
        assert_eq!(USER, 1);
        assert_eq!(ADMIN, 2_305_843_009_213_693_952);
        assert_eq!(SITE_ADMIN, 4_611_686_018_427_387_904);
        assert_eq!(PUBLIC, USER & ADMIN);
        assert_eq!(USER, USER & 3);
    }
}
