#[macro_use]
mod common;

#[test]
fn create_new_task() {
    assert_create_file!(
        [
            "todo new task1",
            "todo --new task1",
            "todo -n task1"
        ] => "issues/task1.md"
    );

    create_file!("todo.toml", r#"
[issue]
dir = "tasks"
format = "{scope}/{priority}.{id}.{name}.{ext}"

[issue.default]
ext = ".md"

[default.new]
scope = "new"
top = "B"
id = { generator = "sequence" }

[generator.sequence]
file = "todo.seq"
"#
    );
    create_file!("todo.seq", "1");

    assert_create_file!(
        "todo new task1" => "tasks/new/B.1.task1.md",
        "todo --new task1" => "tasks/new/B.2.task1.md",
        "todo -n task1" => "tasks/new/B.3.task1.md"
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
        ] => "tasks/cur/A.ID.task\\ 1.txt"
    );

    assert_create_file!(
        "todo -n top:C s:\"\" n:task e:txt" => "tasks/C.4.task.txt",
        "todo new t:C s:\"\" task" => "tasks/C.5.task.md"
    );

//    assert_create_file!(
//        [
//            "todo new --top A --scope cur --id ID --ext txt --name \"task 1\"",
//            "todo new --top A scope:cur id:ID ext:txt \"task 1\"",
//            "todo new --top A scope:cur i:ID e:txt n:\"task 1\"",
//            "todo new t:A s:cur i:ID e:txt \"task 1\"",
//            "todo --new top:A scope:cur id:ID ext:txt name:\"task 1\"",
//            "todo --new top:A scope:cur id:ID ext:txt \"task 1\"",
//            "todo --new top:A scope:cur i:ID e:txt n:\"task 1\"",
//            "todo --new t:A s:cur i:ID e:txt \"task 1\"",
//            "todo -n top:A scope:cur id:ID ext:txt name:\"task 1\"",
//            "todo -n top:A scope:cur id:ID ext:txt \"task 1\"",
//            "todo -n --top A --scope cur -I ID -E txt -N \"task 1\"",
//            "todo -n -T A -S cur -I ID -E txt \"task 1\""
//        ] => "tasks/cur/A.ID.task\\ 1.txt"
//    );
}