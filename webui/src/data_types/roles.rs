/// Default role
///
/// Represents Guest users or publicly accessible pages.
pub const PUBLIC: u32 = 0;

pub const USER: u32 = 1;

pub const CLIENT: u32 = 2;

pub const ADMIN: u32 = 4;

pub const EDITOR: u32 = 8;

pub const CONTRIBUTER: u32 = 16;

pub const VIEWER: u32 = 32;

pub const BILLING: u32 = 64;

pub const OWNER: u32 = 128;

pub const OPERATIONS: u32 = 256;

pub const IT: u32 = 512;

pub const MEMBER: u32 = 1024;

pub const EXTERNAL: u32 = 2048;

pub const INTERNAL: u32 = 4096;

pub const EXECUTIVE: u32 = 8192;

pub const MANAGER: u32 = 16384;

pub const PRODUCTION: u32 = 32768;

pub const CEO: u32 = 65536;

pub const COO: u32 = 131072;

pub const CTO: u32 = 262144;

pub const CFO: u32 = 524288;

pub const CMO: u32 = 1048576;

pub const PRESIDENT: u32 = 2097152;

pub const VICEPRESIDENT: u32 = 4194304;

pub const ASSISTANT: u32 = 8388608;

pub const MARKETING: u32 = 16777216;

pub const PRODUCT: u32 = 33554432;

pub const FINANCE: u32 = 67108864;

pub const HUMANRESOURCES: u32 = 134217728;

pub const BUSINESS: u32 = 268435456;

pub const PERSONNEL: u32 = 536870912;

pub const CUSTOMERSERVICE: u32 = 1073741824;

pub const ADMINISTRATIVE: u32 = 2147483648;

/// Developer Role
///
/// WARNING - This role inherits all other roles, meaning a user assigned as DEVELOPER will have all permissions enabled.
pub const DEVELOPER: u32 = u32::MAX;
