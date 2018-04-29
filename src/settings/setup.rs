use settings::{Settings, Generator};
use todo::issue::{Issue, Content};
use todo::command::{New, List, store::{fs, Create, Find}};

pub trait Setup {
    fn setup(self, settings: &Settings) -> Self;
}

impl<T> Setup for Issue<T>
    where T: Content
{
    fn setup(mut self, settings: &Settings) -> Self {
        for attr in &settings.issue.attrs {
            self.attrs.add_name(attr.key.clone(), attr.aliases.clone());
        }
        self.id_attr_key = settings.issue.id_attr_key.clone();
        self.attrs.default_key = settings.issue.default_attr_key.clone();
        self
    }
}

impl Setup for fs::Create {
    fn setup(mut self, settings: &Settings) -> Self {
        self.format = Some(settings.store.fs.format.clone());
        self.dir = Some(settings.store.fs.dir.clone());
        self.ext = Some(settings.store.fs.ext.clone());
        match settings.store.fs.id_generator.as_ref() {
            Generator::SEQUENCE => {
                self.id_generator = Some(fs::SequenceGenerator {
                    required: settings.generator.sequence.required,
                    file: Some(settings.generator.sequence.file.clone())
                })
            },
            "" => self.id_generator = None,
            generator => panic!("Unsupported generator type `{}`", generator)
        }
        self
    }
}

impl Setup for fs::Find {
    fn setup(mut self, settings: &Settings) -> Self {
        self.all = settings.store.fs.find_all;
        self.dir = settings.store.fs.dir.clone();
        self
    }
}

impl<T> Setup for New<T>
    where T: Create
{
    fn setup(mut self, settings: &Settings) -> Self {
        let command = &settings.command;
        if let Some(ref default_attrs) = command.new.default_attrs {
            for (key, value) in default_attrs.iter() {
                self.issue.attrs.set_attr_by_alias(key.as_str(), value.clone())
                    .expect(&format!("Attribute `{}`, specified in [{}] is not found for command `{}`",
                                     key, stringify!(command.new.default_attrs), stringify!(New)));
            }
        }
        self
    }
}

impl<T> Setup for List<T>
    where T: Find
{
    fn setup(self, _settings: &Settings) -> Self {
        self
    }
}