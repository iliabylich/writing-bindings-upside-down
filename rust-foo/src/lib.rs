pub fn foo(s: String) -> Vec<char> {
    s.chars().filter(|c| !c.is_ascii()).collect()
}

#[test]
fn test_foo() {
    let s = String::from("abc😋中国");
    let chars = foo(s);
    assert_eq!(chars, vec!['😋', '中', '国']);
}
