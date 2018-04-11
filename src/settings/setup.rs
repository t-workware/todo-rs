use settings::{Settings, Generator};
use todo::command::{New, Top, Scope, store::fs, store::Create};

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
                    file: Some(settings.generator.sequence.file.clone())
                })
            },
            "" => self.id_generator = None,
            generator => panic!("Unsupported generator type `{}`", generator)
        }
        self
    }
}

impl<T> Setup for New<T>
    where T: Create
{
    fn setup(mut self, settings: &Settings) -> Self {
        self.top = Some(Top(settings.command.new.top.clone()));
        self.scope = Some(Scope(settings.command.new.scope.clone()));
        self
    }
}