mod element;
pub use element::*;

#[test]
fn create_svg() -> Result<(), std::io::Error> {
    use element::{TagAndAttributes, TagName};

    let mut buffer = Vec::new();

    let toe = TagAndAttributes::new(TagName::LINE)
        .with_attr(Attribute::ID, "spqr")
        .with_attr(Attribute::X, 12)
        .with_attr(Attribute::Y, "100%");

    toe.write_opening(&mut buffer, true)?;

    println!(
        "TOE={}",
        std::str::from_utf8(buffer.as_slice()).unwrap().to_string()
    );

    Ok(())
}
