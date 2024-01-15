/// rust list to lua table (vector).
pub fn to_lua_table(v: &[&str]) -> String {
    let wrap = v.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>();
    ["{", wrap.join(",").as_ref(), "}"].join("")
}

/// rust list to lua table (dictionary).
pub fn to_lua_flag_table(v: &[&str], default: bool) -> String {
    let default = if default { "true" } else { "false" };
    let wrap = v
        .iter()
        .map(|s| format!("[\"{}\"]={}", s, default))
        .collect::<Vec<_>>();
    ["{", wrap.join(",").as_ref(), "}"].join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(arg, exp,
        case(vec![], r#"{}"#),
        case(vec!["a"], r#"{"a"}"#),
        case(vec!["a","b"], r#"{"a","b"}"#),
    )]
    fn test_to_lua_table(arg: Vec<&str>, exp: String) {
        let act = to_lua_table(&arg);

        assert_eq!(exp, act);
    }

    #[rstest(arg, default, exp,
        case(vec![], true, r#"{}"#),
        case(vec![], false, r#"{}"#),
        case(vec!["a"],true, r#"{["a"]=true}"#),
        case(vec!["a"],false, r#"{["a"]=false}"#),
        case(vec!["a","b"],true, r#"{["a"]=true,["b"]=true}"#),
        case(vec!["a","b"],false, r#"{["a"]=false,["b"]=false}"#),
    )]
    fn test_to_lua_flag_table(arg: Vec<&str>, default: bool, exp: String) {
        let act = to_lua_flag_table(&arg, default);

        assert_eq!(exp, act);
    }
}
