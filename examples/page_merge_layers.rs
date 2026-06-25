use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Merge all layers on the page into a single layer with the specified new layer name
    pdf.page_merge_layers(1, "New Layer Name")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_merge_layers.pdf")?;

    Ok(())
}
