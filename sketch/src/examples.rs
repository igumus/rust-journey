fn get_largest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn test_examples() {
    let val = get_largest_string("selam", "selami");
    assert_eq!(val, "selami");
    let s2 = " lkajsdlfkj jlaksdjflkajdflks";
    let val = get_largest_string("selami", s2);
    assert_eq!(val, s2);
}
