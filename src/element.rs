use std::{collections::HashMap, fmt::Display, io::Error as IOError};

pub trait Element {}

pub struct TagAndAttributes {
    tag_name: TagName,
    attributes: HashMap<Attribute, String>,
}

pub enum TagName {
    DOCUMENT,
    GROUP,
    LINE,
    TEXT,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Attribute {
    VIEWBOX,
    X,
    Y,
    ID,
    CLASS,
}

impl TagAndAttributes {
    pub fn new(tag_name: TagName) -> Self {
        Self {
            tag_name,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attr<V>(mut self, name: Attribute, value: V) -> Self
    where
        V: Display,
    {
        self.attributes.insert(name, format!("{}", value));

        self
    }

    pub fn write_opening(
        &self,
        buffer: &mut impl std::io::Write,
        close: bool,
    ) -> Result<(), IOError> {
        buffer.write_fmt(format_args!("<{}", self.tag_name.to_str()))?;

        for (name, value) in self.attributes.iter() {
            buffer.write_fmt(format_args!(" {}='{}'", name.to_str(), value))?;
        }

        if close {
            buffer.write(" /".as_bytes())?;
        }

        buffer.write(">".as_bytes())?;

        Ok(())
    }

    pub fn write_closing(&self, buffer: &mut impl std::io::Write) -> Result<(), IOError> {
        buffer.write_fmt(format_args!("</{}>", self.tag_name.to_str()))
    }
}

impl TagName {
    fn to_str(&self) -> &str {
        match self {
            TagName::DOCUMENT => "svg",
            TagName::GROUP => "g",
            TagName::LINE => "line",
            TagName::TEXT => "text",
        }
    }
}

impl Attribute {
    fn to_str(&self) -> &str {
        match self {
            Attribute::VIEWBOX => "viewbox",
            Attribute::X => "x",
            Attribute::Y => "y",
            Attribute::ID => "id",
            Attribute::CLASS => "class",
        }
    }
}
