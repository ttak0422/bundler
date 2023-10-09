/// rust list to lua table.
pub fn to_lua_table(v: &[&str]) -> String {
    let wrap = v.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>();
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
}
