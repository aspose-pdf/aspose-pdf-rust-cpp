use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Return character count in PDF-document
    let count = pdf.character_count()?;

    // Print the character count
    println!("Character count: {}", count);

    Ok(())
}
