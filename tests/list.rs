#[macro_use]
mod common;

use std::env;

#[test]
fn create_new_task() {
    create_file!("target/test_list/issues/task1.md", "");

    assert_output!(
        [
            "todo list",
            "todo list task1",
            "todo --list",
            "todo --list task1",
            "todo -l",
            "todo -l task1"
        ] => "target/test_list/issues/task1.md"
    );

    delete_file!("target/test_list/issues/task1.md");

    assert_output!(
        [
            "todo list",
            "todo list task1",
            "todo --list",
            "todo --list task1",
            "todo -l",
            "todo -l task1"
        ] => ""
    );
}