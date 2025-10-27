use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Return word count in PDF-document
    let count = pdf.word_count()?;

    // Print the word count
    println!("Word count: {}", count);

    Ok(())
}
