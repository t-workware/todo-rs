#[macro_use]
mod common;

use std::{env, fs};

#[test]
fn create_new_issue() {
    env::set_var("TODO_HOME", "./");
    let _ = fs::remove_dir_all("target/test_new");

    assert_create_file!(
        [
            "todo new task1",
            "todo --new task1",
            "todo -n task1"
        ] => "issues/task1.md"
    );

    create_file!("target/test_new/todo.toml", r#"
[store.fs]
dir = "target/test_new/tasks"
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

    run!("todo -n t:A s:cur i:ID context:test task");
    assert_content!("target/test_new/tasks/cur/A.ID.task.md", "#[context: test]\n");
    delete_file!("target/test_new/tasks/cur/A.ID.task.md");

    run!("todo -n p: s: i: context:test time:\"2 free\" task");
    assert_content!("target/test_new/tasks/task.md", "#[context: test]\n#[time: 2 free]\n");
    delete_file!("target/test_new/tasks/task.md");

    create_file!("target/test_new/todo.toml", r#"
[store.fs]
dir = "target/test_new/tasks"
format = "{scope:/}{name}{.:ext}"
ext = "md"

[issue.attrs]
name = ["n", "title"]
context = ["ctx"]
assign = ["a", "as"]

[issue]
attrs_order = ["context", "assign"]

[command.new.default_attrs]
id = "ID"
priority = "B"
"#
    );

    run!("todo new s: id: ctx:test a:User title:task");
    assert_content!("target/test_new/tasks/task.md", "#[context: test]\n#[assign: User]\n#[priority: B]\n");
    delete_file!("target/test_new/tasks/task.md");
}