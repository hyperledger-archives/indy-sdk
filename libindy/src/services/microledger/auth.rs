use services::microledger::constants::*;
use std::collections::HashSet;

pub struct Auth {}

impl Auth {
    // TODO: have a static list of auths
    pub fn is_valid_auth(auth: &str) -> bool {
        match auth {
            AUTHZ_ALL => true,
            AUTHZ_ADD_KEY => true,
            AUTHZ_REM_KEY => true,
            AUTHZ_MPROX => true,
            _ => false
        }
    }

    pub fn get_all() -> HashSet<String> {
        let mut s: HashSet<String> = HashSet::new();
        s.insert(AUTHZ_ADD_KEY.to_string());
        s.insert(AUTHZ_REM_KEY.to_string());
        s.insert(AUTHZ_MPROX.to_string());
        s
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_valid_auths() {
        let a1 = "all1";
        let a2 = "al";
        let a3 = "addkey";
        let a4 = "remkey";
        let a5 = "m_prox";
        let a6 = "all";
        let a7 = "add_key";
        let a8 = "rem_key";
        let a9 = "mprox";
        assert_eq!(Auth::is_valid_auth(a1), false);
        assert_eq!(Auth::is_valid_auth(a2), false);
        assert_eq!(Auth::is_valid_auth(a3), false);
        assert_eq!(Auth::is_valid_auth(a4), false);
        assert_eq!(Auth::is_valid_auth(a5), false);
        assert_eq!(Auth::is_valid_auth(a6), true);
        assert_eq!(Auth::is_valid_auth(a7), true);
        assert_eq!(Auth::is_valid_auth(a8), true);
        assert_eq!(Auth::is_valid_auth(a9), true);
    }

    #[test]
    fn test_get_all_auths() {
        let expected: HashSet<String> = [AUTHZ_ADD_KEY.to_string(), AUTHZ_REM_KEY.to_string(),
            AUTHZ_MPROX.to_string()].iter().cloned().collect();
        assert_eq!(Auth::get_all(), expected);
    }
}
