use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Specify the page number (1-based index)
    let page_number = 1;

    // Return character count on the specified page
    let count = pdf.page_character_count(page_number)?;

    // Print the character count
    println!("Character count on page {}: {}", page_number, count);

    Ok(())
}
