use types::Str;

pub struct Cmd {
    pub name: Str,
    pub short: Str,
    pub desc: Str
}

macro_rules! commands {
    ($($const_name:ident [$name:ident, -$short:ident, --$long:ident] - $desc:tt),*) => {
        impl Cmd {
            $(pub const $const_name: Cmd = Cmd { name: stringify!($name), short: stringify!($short), desc: $desc };)*
        }
    };
}

commands! {
    NEW [new, -n, --new] - "Create new issue",
    LIST [list, -l, --list] - "List issues"
}