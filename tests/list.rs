#[macro_use]
mod common;

#[test]
fn list_issues() {
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

    assert_output!(
        [
            "todo list task2",
            "todo --list task2",
            "todo -l task2"
        ] => ""
    );

    create_file!("target/test_list/issues/new/task2.md", "");

    assert_output!(
        [
            "todo list",
            "todo --list",
            "todo -l"
        ] => r#"
target/test_list/issues/task1.md
target/test_list/issues/new/task2.md
"#
    );

    assert_output!(
        [
            "todo list task2",
            "todo --list task2",
            "todo -l task2"
        ] => "target/test_list/issues/new/task2.md"
    );

    delete_file!("target/test_list/issues/task1.md");
    delete_file!("target/test_list/issues/new/task2.md");

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