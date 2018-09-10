mod testing {
    pub fn test_plain_macro() -> String {
        use example_api::module::is_a_test;
        is_a_test!(message=> "there".to_owned(), name=>"hi".to_owned())
    }

    pub fn test_nested_macro() -> String {
        use example_api::module;
        module::is_a_test!(message=> "there".to_owned(), name=>"hi".to_owned())
    }

    pub fn test_plain_fn() -> String {
        use example_api::module::is_a_test;
        is_a_test(Some("hi".to_owned()), "there".to_owned())
    }

    pub fn test_nested_fn() -> String {
        use example_api::module;
        module::is_a_test(Some("hi".to_owned()), "there".to_owned())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_named_works() {

        assert_eq!(
            "1) Hello hi, there The end.",
        crate::testing::test_plain_macro(),
        );

        assert_eq!(
            "1) Hello hi, there The end.",
        crate::testing::test_nested_macro(),
        );

        assert_eq!(
            "1) Hello hi, there The end.",
        crate::testing::test_plain_fn(),
        );

        assert_eq!(
            "1) Hello hi, there The end.",
        crate::testing::test_nested_fn(),
        );

    }
}
