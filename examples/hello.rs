use asposepdf::{Document, PageSize};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new PDF-document
    let pdf = Document::new()?;

    // Add a new page
    pdf.page_add()?;

    // Set the size of the first page to A4
    pdf.page_set_size(1, PageSize::A4)?;

    // Add "Hello World!" text to the first page
    pdf.page_add_text(1, "Hello World!")?;

    // Save the PDF-document as "hello.pdf"
    pdf.save_as("hello.pdf")?;

    println!("Saved PDF-document: hello.pdf");

    Ok(())
}
