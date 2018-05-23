use settings::{Generator, Settings};
use todo::command::{store::{fs, Create, Find}, List, New};
use todo::issue::{Content, Issue};

pub trait Setup {
    fn setup(self, settings: &Settings) -> Self;
}

impl<T> Setup for Issue<T>
where
    T: Content,
{
    fn setup(mut self, settings: &Settings) -> Self {
        let attrs_order = settings.issue.attrs_order.as_ref();
        if let Some(attrs_order) = attrs_order {
            for attr in attrs_order {
                if let Some(aliases) = settings.issue.attrs.get(attr) {
                    let key = self.attrs.add_key(attr.as_str());
                    self.attrs
                        .add_aliases(key.as_str(), aliases)
                        .expect("Setup ordered aliases error");
                }
            }
        }
        for (attr, aliases) in &settings.issue.attrs {
            if let Some(attrs_order) = attrs_order {
                if attrs_order.contains(attr) {
                    continue;
                }
            }
            let key = self.attrs.add_key(attr.as_str());
            self.attrs
                .add_aliases(key.as_str(), aliases)
                .expect("Setup unordered aliases error");
        }
        let key = self.attrs.add_key(&settings.issue.id_attr_key);
        self.id_attr_key = (*key).clone();
        let key = self.attrs.add_key(&settings.issue.default_attr_key);
        self.attrs.default_key = key;
        self
    }
}

impl Setup for fs::Create {
    fn setup(mut self, settings: &Settings) -> Self {
        self.attrs.set_attr_value(
            fs::CreateAttr::IssuesDir.key(),
            settings.store.fs.issues_dir.clone(),
        );
        self.attrs.set_attr_value(
            fs::CreateAttr::Format.key(),
            settings.store.fs.format.clone(),
        );
        self.attrs.set_attr_value(
            fs::CreateAttr::Ext.key(),
            settings.store.fs.ext.clone()
        );

        for (key, aliases) in &settings.store.fs.attrs {
            let _ = self.attrs.add_aliases(key.as_str(), aliases);
        }

        match settings.store.fs.id_generator.as_ref() {
            Generator::SEQUENCE => {
                self.id_generator = Some(fs::SequenceGenerator {
                    required: settings.generator.sequence.required,
                    file: Some(settings.generator.sequence.file.clone()),
                })
            }
            "" => self.id_generator = None,
            generator => panic!("Unsupported generator type `{}`", generator),
        }
        self
    }
}

impl Setup for fs::Find {
    fn setup(mut self, settings: &Settings) -> Self {
        if settings.store.fs.find_all {
            self.attrs.set_attr_value(
                fs::FindAttr::All.key(),
                true.to_string()
            );
        }
        self.attrs.set_attr_value(
            fs::FindAttr::IssuesDir.key(),
            settings.store.fs.issues_dir.clone(),
        );
        for (key, aliases) in &settings.store.fs.attrs {
            let _ = self.attrs.add_aliases(key.as_str(), aliases);
        }
        self
    }
}

impl<T> Setup for New<T>
where
    T: Create,
{
    fn setup(mut self, settings: &Settings) -> Self {
        let command = &settings.command;
        if let Some(ref default_attrs) = command.new.default_attrs {
            for (key, value) in default_attrs.iter() {
                self.issue.attrs.set_attr_value(key.as_str(), value.clone());
            }
        }
        self
    }
}

impl<T> Setup for List<T>
where
    T: Find,
{
    fn setup(self, _settings: &Settings) -> Self {
        self
    }
}
