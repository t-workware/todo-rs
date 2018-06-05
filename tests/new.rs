#[macro_use]
mod common;

use std::{env, fs};

#[test]
fn create_new_issue() {
    env::set_var("TODO_HOME", "./");
    fs::remove_dir_all("target/test_new")
        .expect("Can't remove test_new dir");

    //
    // Testing creation by name
    //

    assert_create_file!(
        [
            "todo new task1",
            "todo --new task1",
            "todo -n task1"
        ] => "issues/task1.md"
    );

    //
    // Testing creation with default attrs
    //

    create_file!("target/test_new/todo.toml", r#"
[store.fs]
issues_dir = "target/test_new/tasks"
format = "{scope:/}{priority:.}{id:.}{name}{.:ext}"
ext = "md"
id_generator = "sequence"

[command.new.default_attrs]
scope = "new"
priority = "B"

[generator.sequence]
required = true
file = "target/test_new/todo.seq"
"#
    );
    env::set_var("TODO_CONFIG_FILE_NAME", "target/test_new/todo.toml");
    create_file!("target/test_new/todo.seq", "1");

    assert_create_file!(
        "todo new task1" => "target/test_new/tasks/new/B.1.task1.md",
        "todo --new task1" => "target/test_new/tasks/new/B.2.task1.md",
        "todo -n task1" => "target/test_new/tasks/new/B.3.task1.md"
    );

    //
    // Testing creation with described attrs
    //

    assert_create_file!(
        [
            "todo new top:A scope:cur id:ID ext:txt name:\"task 1\"",
            "todo new top:A scope:cur id:ID ext:txt \"task 1\"",
            "todo new top:A scope:cur i:ID ext:txt n:\"task 1\"",
            "todo new t:A s:cur i:ID ext:txt \"task 1\"",
            "todo --new top:A scope:cur id:ID ext:txt name:\"task 1\"",
            "todo --new top:A scope:cur id:ID ext:txt \"task 1\"",
            "todo --new top:A scope:cur i:ID ext:txt n:\"task 1\"",
            "todo --new t:A s:cur i:ID ext:txt \"task 1\"",
            "todo -n top:A scope:cur id:ID ext:txt name:\"task 1\"",
            "todo -n top:A scope:cur id:ID ext:txt \"task 1\"",
            "todo -n top:A scope:cur i:ID ext:txt n:\"task 1\"",
            "todo -n t:A s:cur i:ID ext:txt \"task 1\""
        ] => "target/test_new/tasks/cur/A.ID.task 1.txt"
    );

    assert_create_file!(
        "todo -n top:C s:\"\" n:task ext:txt" => "target/test_new/tasks/C.4.task.txt",
        "todo new t:C s:\"\" task" => "target/test_new/tasks/C.5.task.md"
    );

    //
    // Testing creation with new attrs
    //

    run!("todo -n t:A s:cur i:ID context:test task");
    assert_content!(
        "target/test_new/tasks/cur/A.ID.task.md",
        "#[context: test]\n"
    );
    delete_file!("target/test_new/tasks/cur/A.ID.task.md");

    //
    // Testing new attrs order
    //

    run!("todo -n p: s: i: context:test time:\"2 free\" task");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[context: test]\n#[time: 2 free]\n"
    );
    delete_file!("target/test_new/tasks/task.md");

    run!("todo -n p: s: i: time:\"2 free\" context:test task");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[time: 2 free]\n#[context: test]\n"
    );
    delete_file!("target/test_new/tasks/task.md");

    //
    // Testing creation with custom attrs aliases and cli ordering
    //

    create_file!("target/test_new/todo.toml", r#"
[store.fs]
issues_dir = "target/test_new/tasks"
format = "{scope:/}{name}{.:ext}"
ext = "md"

[issue.attrs]
name = ["n", "title"]
context = ["ctx"]
assign = ["a", "as"]

[issue]
attrs_order = ["id", "priority"]

[command.new.default_attrs]
id = "ID"
priority = "B"
"#
    );

    run!("todo new a:User title:task ctx:test");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[id: ID]\n#[priority: B]\n#[assign: User]\n#[context: test]\n"
    );
    delete_file!("target/test_new/tasks/task.md");

    run!("todo new id: a:User title:task priority:C ctx:test");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[priority: C]\n#[assign: User]\n#[context: test]\n"
    );
    delete_file!("target/test_new/tasks/task.md");

    //
    // Testing creation with custom attrs aliases and config order
    //

    create_file!("target/test_new/todo.toml", r#"
[store.fs]
issues_dir = "target/test_new/tasks"
format = "{scope:/}{name}{.:ext}"
ext = "md"

[issue.attrs]
name = ["n", "title"]
context = ["ctx"]
assign = ["a", "as"]

[issue]
attrs_order = ["id", "priority", "context", "assign"]

[command.new.default_attrs]
id = "ID"
priority = "B"
"#
    );

    run!("todo new id: a:User title:task ctx:test");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[priority: B]\n#[context: test]\n#[assign: User]\n"
    );
    delete_file!("target/test_new/tasks/task.md");

    run!("todo new top:A id:ID attr1:test a:User title:task attr2:test");
    assert_content!(
        "target/test_new/tasks/task.md",
        "#[id: ID]\n#[priority: A]\n#[assign: User]\n#[attr1: test]\n#[attr2: test]\n"
    );
    delete_file!("target/test_new/tasks/task.md");
}
