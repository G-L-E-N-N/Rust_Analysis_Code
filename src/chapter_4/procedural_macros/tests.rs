use test_macros::generate_tests;

generate_tests! {
    test_add => 1 + 2 == 3,
    test_contains => vec![10, 20, 30].contains(&20),
}
