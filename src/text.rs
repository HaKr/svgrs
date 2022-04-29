use crate::element::{CoreAttributes, Element, LengthOrPercentage};

use super::{Attribute, StylingAttributes, TagAndAttributes, TagName};

pub struct Text {
    pub(crate) inner: TagAndAttributes,
    text: String,
}

impl Text {
    pub fn new(x: LengthOrPercentage, y: LengthOrPercentage, text: &str) -> Self {
        Self {
            inner: TagAndAttributes::new(TagName::TEXT)
                .with_attr(Attribute::X, x)
                .with_attr(Attribute::Y, y),
            text: text.to_owned(),
        }
    }
}

core_attributes!(Text);
styling_attributes!(Text);

impl Element for Text {
    fn write(&self, buffer: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.inner.write_opening(buffer, false)?;
        buffer.write(self.text.as_bytes())?;
        self.inner.write_closing(buffer)
    }
}
