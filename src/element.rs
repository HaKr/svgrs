use std::{collections::HashMap, fmt::Display, io::Error as IOError};

pub trait Element {
    fn write(&self, buffer: &mut impl std::io::Write) -> Result<(), std::io::Error>;
}

pub trait CoreAttributes {
    fn id(self, id: &str) -> Self;
    fn tabindex(self, tabindex: i32) -> Self;
}

pub trait StylingAttributes {
    fn add_class(self, class: &str) -> Self;
}

pub struct TagAndAttributes {
    tag_name: TagName,
    pub(crate) attributes: HashMap<Attribute, String>,
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
    TABINDEX,
}

pub enum LengthOrPercentage {
    Number(usize),
    Em(usize),
    Ex(usize),
    Pixels(usize),
    Inch(f32),
    Cm(f32),
    Mm(f32),
    Point(usize),
    Pica(usize),
    Percentage(f32),
}

impl Display for LengthOrPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthOrPercentage::Number(num) => f.write_fmt(format_args!("{}", num)),
            LengthOrPercentage::Em(em) => f.write_fmt(format_args!("{}em", em)),
            LengthOrPercentage::Ex(ex) => f.write_fmt(format_args!("{}ex", ex)),
            LengthOrPercentage::Pixels(px) => f.write_fmt(format_args!("{}px", px)),
            LengthOrPercentage::Inch(inch) => f.write_fmt(format_args!("{}in", inch)),
            LengthOrPercentage::Cm(cm) => f.write_fmt(format_args!("{}cm", cm)),
            LengthOrPercentage::Mm(mm) => f.write_fmt(format_args!("{}mm", mm)),
            LengthOrPercentage::Point(pt) => f.write_fmt(format_args!("{}pt", pt)),
            LengthOrPercentage::Pica(pc) => f.write_fmt(format_args!("{}pc", pc)),
            LengthOrPercentage::Percentage(perc) => f.write_fmt(format_args!("{}%", perc)),
        }
    }
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

    pub(crate) fn set_attr<V>(&mut self, name: Attribute, value: V)
    where
        V: Display,
    {
        self.attributes.insert(name, format!("{}", value));
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
            Attribute::TABINDEX => "tabindex",
        }
    }
}

macro_rules! core_attributes {
    ($struct:ident) => {
        impl CoreAttributes for $struct {
            fn id(mut self, id: &str) -> Self {
                self.inner.set_attr(Attribute::ID, id);

                self
            }

            fn tabindex(mut self, tabindex: i32) -> Self {
                self.inner.set_attr(Attribute::TABINDEX, tabindex);

                self
            }
        }
    };
}

macro_rules! styling_attributes {
    ($struct:ident) => {
        impl StylingAttributes for $struct {
            fn add_class(mut self, id: &str) -> Self {
                let entry = self
                    .inner
                    .attributes
                    .entry(Attribute::CLASS)
                    .or_insert_with(|| String::from(""));
                let sep = String::from(if (*entry).len() > 0 { " " } else { "" });
                *entry = format!("{}{}{}", entry, sep, id);

                self
            }
        }
    };
}

macro_rules! write_closed {
    ($struct:ident) => {
        impl Element for $struct {
            fn write(&self, buffer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
                self.inner.write_opening(buffer, true)
            }
        }
    };
}
