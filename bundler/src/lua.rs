/// rust list to lua table.
pub fn to_lua_table(v: &[&str]) -> String {
    let wrap = v.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>();
    ["{", wrap.join(",").as_ref(), "}"].join("")
}
