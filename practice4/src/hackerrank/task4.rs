pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .filter(|&&d| {
            let position = a + d;
            position >= s && position <= t
        })
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&d| {
            let position = b + d;
            position >= s && position <= t
        })
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruits() {
        let result = count_apples_and_oranges(
            7,
            11,
            5,
            15,
            vec![-2, 2, 1],
            vec![5, -6],
        );

        assert_eq!(result, (1, 1));
    }
}
