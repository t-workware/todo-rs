#[macro_use]
mod common;

use std::env;

#[test]
fn create_new_task() {
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
format = "{scope:/}{top:.}{id:.}{name}{.:ext}"
ext = "md"
id_generator = "sequence"

[command.new]
scope = "new"
top = "B"

[generator.sequence]
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
            "todo new top:A scope:cur i:ID e:txt n:\"task 1\"",
            "todo new t:A s:cur i:ID e:txt \"task 1\"",
            "todo --new top:A scope:cur id:ID ext:txt name:\"task 1\"",
            "todo --new top:A scope:cur id:ID ext:txt \"task 1\"",
            "todo --new top:A scope:cur i:ID e:txt n:\"task 1\"",
            "todo --new t:A s:cur i:ID e:txt \"task 1\"",
            "todo -n top:A scope:cur id:ID ext:txt name:\"task 1\"",
            "todo -n top:A scope:cur id:ID ext:txt \"task 1\"",
            "todo -n top:A scope:cur i:ID e:txt n:\"task 1\"",
            "todo -n t:A s:cur i:ID e:txt \"task 1\""
        ] => "target/test_new/tasks/cur/A.ID.task 1.txt"
    );

    assert_create_file!(
        "todo -n top:C s:\"\" n:task e:txt" => "target/test_new/tasks/C.4.task.txt",
        "todo new t:C s:\"\" task" => "target/test_new/tasks/C.5.task.md"
    );
}