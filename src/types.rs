use std::fmt;

// #[derive(Debug)]
// pub struct AuthResponse {
//     pub application_id: String,
//     pub application_name: String,
//     pub tenant_id: String,
// }

#[derive(Debug)]
pub struct AuthInfo {
    pub application_id: String,
    pub tenant_id: String,
}

#[derive(Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
}

#[derive(Copy, Clone)]
pub struct XcodeVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Debug for XcodeVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Xcode version: {}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}
