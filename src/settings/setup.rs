use settings::{Settings, Generator};
use todo::command::{New, List, store::{fs, Create, Find}};
use todo::issue::{Top, Scope};

pub trait Setup {
    fn setup(self, settings: &Settings) -> Self;
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
        self.dir = settings.store.fs.dir.clone();
        self
    }
}

impl<T> Setup for New<T>
    where T: Create
{
    fn setup(mut self, settings: &Settings) -> Self {
        self.issue.top = Some(Top(settings.command.new.top.clone()));
        self.issue.scope = Some(Scope(settings.command.new.scope.clone()));
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