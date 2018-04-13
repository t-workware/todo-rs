pub trait Content {
    fn content(&self) -> String;
}

impl Content for String {
    fn content(&self) -> String {
        self.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Top(pub String);

#[derive(Clone, Debug, Default)]
pub struct Scope(pub String);

#[derive(Clone, Debug, Default)]
pub struct Id(pub String);


#[derive(Clone, Debug, Default)]
pub struct Issue<T: Content> {
    pub id: Option<Id>,
    pub top: Option<Top>,
    pub scope: Option<Scope>,
    pub name: Option<String>,
    pub content: Option<T>
}