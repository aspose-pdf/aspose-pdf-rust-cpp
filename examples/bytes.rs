use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new PDF-document
    let pdf = Document::new()?;

    // Return the contents of the PDF-document as a byte vector
    let data = pdf.bytes()?;

    // Print length of the byte vector
    println!("Length: {}", data.len());

    Ok(())
}
