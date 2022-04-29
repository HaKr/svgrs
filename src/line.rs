use crate::element::{CoreAttributes, LengthOrPercentage};

use super::{Attribute, Element, StylingAttributes, TagAndAttributes, TagName};

pub struct Line {
    pub(crate) inner: TagAndAttributes,
}

impl Line {
    pub fn new(x: LengthOrPercentage, y: LengthOrPercentage) -> Self {
        Self {
            inner: TagAndAttributes::new(TagName::LINE)
                .with_attr(Attribute::X, x)
                .with_attr(Attribute::Y, y),
        }
    }
}

core_attributes!(Line);
styling_attributes!(Line);
write_closed!(Line);
