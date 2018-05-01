use todo::attrs::Attrs;

pub trait Content {
    fn content(&self) -> String;
}

impl Content for String {
    fn content(&self) -> String {
        self.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Issue<T: Content> {
    pub id_attr_key: String,
    pub attrs: Attrs,
    pub content: Option<T>
}

impl<T: Content> Issue<T> {
    pub fn get_id(&self) -> Option<&String> {
        self.attrs.attr_value(&self.id_attr_key)
    }
}