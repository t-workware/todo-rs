#[macro_use]
mod common;

use std::{env, fs};

#[test]
fn list_issues() {
    env::set_var("TODO_HOME", "./");
    fs::remove_dir_all("target/test_list")
        .expect("Can't remove test_list dir");

    //
    // Test simple listing
    //

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

    //
    // Test filtering by regex
    //

    assert_output!(
        [
            "todo list task2",
            "todo --list task2",
            "todo -l task2",
            "todo list .*2",
            "todo --list .*2",
            "todo -l .*2"
        ] => "target/test_list/issues/new/task2.md"
    );

    assert_output!(
        [
            "todo list .*3",
            "todo --list .*3",
            "todo -l .*3"
        ] => ""
    );

    assert_output!(
        [
            "todo list task",
            "todo --list task",
            "todo -l task",
            "todo list \\.md$",
            "todo --list \\.md$",
            "todo -l \\.md$"
        ] => r#"
target/test_list/issues/task1.md
target/test_list/issues/new/task2.md
"#
    );

    assert_output!(
        [
            "todo list ^task$",
            "todo --list ^task$",
            "todo -l ^task$"
        ] => ""
    );

    //
    // Test filtering hidden issues
    //

    create_file!("target/test_list/issues/.fin/task0.md", "");
    create_file!("target/test_list/issues/.task3.md", "");

    assert_output!(
        [
            "todo list",
            "todo list all:false",
            "todo list a:f",
            "todo --list",
            "todo --list all:not",
            "todo --list a:no",
            "todo -l all:f",
            "todo -l a:n",
            "todo -l a:0"
        ] => r#"
target/test_list/issues/task1.md
target/test_list/issues/new/task2.md
"#
    );

    assert_output!(
        [
            "todo list a:",
            "todo --list all:",
            "todo -l a:t",
            "todo -l all:+"
        ] => r#"
target/test_list/issues/task1.md
target/test_list/issues/new/task2.md
target/test_list/issues/.fin/task0.md
target/test_list/issues/.task3.md
"#
    );

    delete_file!("target/test_list/issues/task1.md");
    delete_file!("target/test_list/issues/new/task2.md");
    delete_file!("target/test_list/issues/.fin/task0.md");
    delete_file!("target/test_list/issues/.task3.md");

    assert_output!(
        [
            "todo list",
            "todo list task1",
            "todo list all:",
            "todo --list",
            "todo --list task1",
            "todo --list a:true",
            "todo -l",
            "todo -l task1",
            "todo -l a:"
        ] => ""
    );
}
