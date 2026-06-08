fn staircase(n: usize) -> Vec<String> {
    let mut res = vec![];

    for i in 1..=n {
        res.push(format!("{}{}", " ".repeat(n - i), "#".repeat(i)));
    }

    res
}

#[test]
fn test_staircase() {
    assert_eq!(staircase(3), vec![
        "  #",
        " ##",
        "###",
    ]);
}