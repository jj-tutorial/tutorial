#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/**/*.trycmd")
        .env("JJ_TIMESTAMP", "2025-02-05T22:43:34+00:00")
        .env("JJ_OP_TIMESTAMP", "2025-02-05T22:43:34+00:00")
        .env("TZ", "UTC");
}