use std::path::Path;

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/**/*.trycmd")
        .env("JJ_TIMESTAMP", "2025-02-05T22:43:34+00:00")
        .env("JJ_OP_TIMESTAMP", "2025-02-05T22:43:34+00:00")
        .env("JJ_OP_HOSTNAME", "host.example.com")
        .env("JJ_OP_USERNAME", "steveklabnik")
        .env("TZ", "UTC")
        .env("LANG", "C.UTF-8")
        .env("JJ_TZ_OFFSET_MINS", "660")
        .register_bin("tree", Path::new("/usr/bin/tree"))
        .register_bin("cat", Path::new("/usr/bin/cat"))
        .register_bin("sed", Path::new("/usr/bin/sed"));
}
