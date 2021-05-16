pub fn wd(words: Vec<String>, number: Vec<u64>) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .filter_map(|(i, word)| number.contains(&(i as u64 + 1)).then(|| word.clone()))
        .collect()
}

#[test]
fn test_wd() {
    struct TestCase {
        words: Vec<String>,
        number: Vec<u64>,
        expected: Vec<String>,
    }
    let testcases = vec![
        TestCase {
            words: vec!["a".to_string()],
            number: vec![1],
            expected: vec!["a".to_string()],
        },
        TestCase {
            words: vec!["a".to_string(), "b".to_string()],
            number: vec![1],
            expected: vec!["a".to_string()],
        },
        TestCase {
            words: vec!["abc".to_string(), "def".to_string()],
            number: vec![2],
            expected: vec!["def".to_string()],
        },
        TestCase {
            words: vec!["a".to_string(), "b".to_string()],
            number: vec![1, 2],
            expected: vec!["a".to_string(), "b".to_string()],
        },
        TestCase {
            words: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            number: vec![1, 3],
            expected: vec!["a".to_string(), "c".to_string()],
        },
        TestCase {
            words: vec!["a".to_string(), "b".to_string()],
            number: vec![0, 3],
            expected: vec![],
        },
    ];

    for testcase in testcases {
        assert_eq!(wd(testcase.words, testcase.number), testcase.expected);
    }
}
