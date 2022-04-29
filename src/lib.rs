#[macro_use]
mod element;
pub(crate) use element::*;

mod line;
pub use line::Line;

mod text;
pub use text::Text;

#[cfg(test)]
type TestResult = Result<(), std::io::Error>;

#[cfg(test)]
fn show(toe: &impl Element) -> TestResult {
    let mut buffer = Vec::new();

    toe.write(&mut buffer)?;

    println!(
        "TOE={}",
        std::str::from_utf8(buffer.as_slice()).unwrap().to_string()
    );

    Ok(())
}

#[test]
fn create_line() -> TestResult {
    let toe = Line::new(
        LengthOrPercentage::Number(100),
        LengthOrPercentage::Number(200),
    )
    .id("jjdk")
    .add_class("axis")
    .add_class("x-axis");

    show(&toe)
}

#[test]

fn create_text() -> TestResult {
    let toe = Text::new(
        LengthOrPercentage::Number(20),
        LengthOrPercentage::Number(30),
        "My cat is Grumpy!"
    )
    .id("grumpy-cat")
    .add_class("heavy")
    .add_class("Rrrrrrr");

    show(&toe)
}
