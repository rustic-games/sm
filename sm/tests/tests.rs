extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    //config.target_rustcflags = Some("-L ../target/debug -L ../target/debug/deps".to_string());
    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("run-pass");
    run_mode("compile-fail");
}
