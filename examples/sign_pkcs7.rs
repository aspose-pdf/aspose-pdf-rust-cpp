use asposepdf::Document;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read certificate and image files into byte vectors
    let cert = fs::read("sign.pfx")?;
    let img = fs::read("sign.png")?;

    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Sign a PDF-document using PKCS#7 digital signatures
    pdf.sign_pkcs7(
        1,
        &cert,
        "Pa$$w0rd2023",
        100,
        100,
        70,
        100,
        "Reason",
        "Contact",
        "Location",
        true,
        &img,
        "sample_sign_pkcs7.pdf",
    )?;

    Ok(())
}
