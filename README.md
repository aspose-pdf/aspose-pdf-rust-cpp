# Aspose.PDF for Rust via C++

The package asposepdf is a powerful toolkit that allows developers to manipulate PDF files directly and helps do various tasks for PDF.
Contains unique features for converting PDF to other formats.

## Features

### PDF Processing

- **Create and manage documents**
  - `new`, `open`, `save`, `save_as`, `set_license`
    Create, load, save as, and save PDF-document; apply license keys.
  - `append`, `append_pages`, `merge_documents`, `split_document`, `split`, `split_at_page`, `split_at`
    Append full documents or specific pages; merge multiple PDF-documents; split a PDF-document by page ranges or at a specific page.

- **Page management**
  - `add`, `insert`, `delete`, `count`
    Add, insert, delete, and count pages in a document.

- **Document-level operations**
  - `optimize`, `optimize_resource`, `optimize_file_size`, `grayscale`, `flatten`, `rotate`, `set_background`, `repair`
    Optimize PDF-document layout, size and resources, convert to grayscale, flatten, rotate pages, set background, and repair corrupted documents.
  - `replace_text`, `add_page_num`, `add_text_header`, `add_text_footer`, `add_watermark`
    Replace text, add page numbers, insert custom text in the header or footer, and add watermark.
  - `remove_annotations`, `remove_attachments`, `remove_blank_pages`, `remove_bookmarks`, `remove_hidden_text`, `remove_images`, `remove_tables`, `remove_watermarks`, `remove_text_headers`, `remove_text_footers`, `remove_javascripts`
    Remove annotations, attachments, blank pages, bookmarks, hidden text, images, tables, watermark, headers, footers, and embedded JavaScript code.
  - `embed_fonts`, `unembed_fonts`
    Embed and unembed fonts a PDF-document.

- **Page-level operations**
  - `page_rotate`, `page_set_size`, `page_grayscale`, `page_add_text`, `page_add_watermark`
    Rotate individual pages, set page size, convert pages to grayscale, add text, and add watermark.
  - `page_replace_text`, `page_add_page_num`, `page_add_text_header`, `page_add_text_footer`
    Replace text on a specific page, add page number to a page, and insert custom text in the header or footer of a page.
  - `page_remove_annotations`, `page_remove_hidden_text`, `page_remove_images`, `page_remove_tables`, `page_remove_text_headers`, `page_remove_text_footers`, `page_remove_watermarks`
    Remove annotations, hidden text, images, tables, headers, footers and watermarks on a specific page.

- **Content extraction**
  - `extract_text`, `bytes`
    Retrieve plain text content, and raw data from PDF-document.
  - `export_fdf`, `export_xfdf`, `export_xml`
    Export data from the previously opened PDF-document with AcroForm to FDF, XFDF, or XML formats.

### PDF converting and saving

- **Microsoft Office:**
  - `DOC`, `DOCX`, `XLSX`, `PPTX`
  - `DOCX` with Enhanced Recognition Mode (fully editable tables and paragraphs)

- **Images:**
  - `JPEG`, `PNG`, `BMP`, `TIFF`

- **PDFs:**
  - `N-UP`, `BOOKLET`

- **Others:**
  - `EPUB`, `DICOM`, `SVG`, `SVG(ZIP)`, `XPS`, `TEX`, `TXT`, `MARKDOWN`

### Metadata

- **Product Info:**
  - `about`
    Return metadata information about the Aspose.PDF for Rust via C++ with product name, version, release date, and license status.

### PDF analysis

- **Document statistics:**
  - `word_count`, `character_count`
    Return the number of words and characters in the entire PDF document.

- **Page statistics:**
  - `page_word_count`, `page_character_count`, `page_is_blank`
    Return the number of words and characters on a specific page and check if a page is blank.

### PDF secure

- **Open password-protected:**
  - `open_with_password`
    Open a password-protected PDF-document.

- **Encrypt/decrypt document:**
  - `encrypt`, `decrypt`
    Encrypt and Decrypt PDF-document.

- **Configure access permissions:**
  - `set_permissions`, `get_permissions`
    Set permissions for PDF-document and get current permissions of PDF-document.

## Platforms

Implemented support for Linux x64, macOS x86_64, macOS arm64 and Windows x64 platforms.

The platform-specific version of the dynamic library from the 'lib'-folder in the package's root directory is required for distributing the resulting application:
- *libAsposePDFforRust_linux_amd64.so* for Linux x64 platform
- *libAsposePDFforRust_darwin_arm64.dylib* for macOS arm64 platform
- *libAsposePDFforRust_darwin_amd64.dylib* for macOS x86_64 platform
- *AsposePDFforRust_windows_amd64.dll and AsposePDFforRust_windows_amd64.lib* for Windows x64 platform.

## Installation

### Installation from Aspose website

This package includes a large file which is stored as a bzip2 archive.

1. **Download** the archive **Aspose.PDF for Rust via C++** from the official Aspose website.
   The latest (most recent) version is listed at the top and is downloaded by default when you click the **Download** button.
   It is recommended to use this latest version. Only download a previous version if needed.
   Example: `Aspose.PDF-for-Rust-via-CPP-25.6.zip`

   The archive filename format is: `Aspose.PDF-for-Rust-via-CPP-YY.M.zip`, where:
   - `YY` = last two digits of the year (e.g., `25` for 2025)
   - `M` = month number from `1` to `12`

2. **Extract** the archive to your chosen directory `{path}` using a suitable tool:
   - On Linux/macOS:
     ```bash
     unzip Aspose.PDF-for-Rust-via-CPP-YY.M.zip -d {path}
     ```
   - On Windows, use built-in Explorer extraction or any unzip tool (7-Zip, WinRAR).

3. **Add** the library as a dependency in your Rust project. You can do this in two ways:

   - **Using the command line:**
     ```bash
     cargo add asposepdf --path {path}/asposepdf
     ```

   - **Manually editing `Cargo.toml`:**
     Open your project's `Cargo.toml` and add the following under `[dependencies]`:
     ```toml
     [dependencies]
     asposepdf = { path = "{path}/asposepdf" }
     ```

4. **Build** your project (`cargo build`). On the first build, the dynamic library for your platform will be unpacked automatically from the `.bz2` archive in the `lib` folder. This may cause a short delay.

> **Notes**
> - The `lib` folder contains all platform-specific `.bz2` archives with corresponding `.sha256` checksum files.
> - If the checksum file is missing or invalid, the build will fail.
> - Update the library by replacing the extracted files with a newer archive version.

### Installation from GitHub

This package includes precompiled native libraries (`.dll`, `.so`, `.dylib`) which are stored as compressed `.bz2` archives inside the GitHub repository.

1. **Add** the library as a dependency in your Rust project. You can do this in two ways:

   - **Using the command line:**
     ```bash
     cargo add asposepdf --git https://github.com/aspose-pdf/aspose-pdf-rust-cpp.git
     ```

   - **Manually editing `Cargo.toml`:**
     Open your project's `Cargo.toml` and add the following under `[dependencies]`:
     ```toml
     [dependencies]
     asposepdf = { git = "https://github.com/aspose-pdf/aspose-pdf-rust-cpp.git" }
     ```

		> **Note:** To use a specific release version, you can specify a tag:
		>
		> ```toml
		> asposepdf = { git = "https://github.com/aspose-pdf/aspose-pdf-rust-cpp.git", tag = "v1.25.7" }
		> ```

2. **Build** your project (`cargo build`). On the first build, the appropriate dynamic library for your platform will be automatically unpacked from the `.bz2` archive in the `lib` folder. This may cause a short delay.

> **Notes**
> - You do not need to manually download or extract any files — everything is included in the GitHub repository.
> - All `.bz2` archives have matching `.sha256` checksum files. The checksum is verified before unpacking.
> - If the checksum verification fails or the archive is missing, the build will fail with a detailed error.
> - The build script links the appropriate native library and ensures runtime availability using platform-specific options.

### Installation from crates.io

This package is available on [crates.io](https://crates.io/crates/asposepdf) and includes a build script that automatically extracts the required native library (`.dll`, `.so`, or `.dylib`) from a compressed `.bz2` archive during the build process.

1. **Add** the library as a dependency in your Rust project. You can do this in two ways:

   - **Using the command line:**
     ```bash
     cargo add asposepdf
     ```

   - **Manually editing `Cargo.toml`:**
     Open your project's `Cargo.toml` and add the following under `[dependencies]`:
     ```toml
     [dependencies]
     asposepdf = "1.25.7"
     ```
	> **Note:** The crates.io package requires you to provide the native dynamic libraries yourself (the .dll, .so, .dylib files).

2. **Set the path** to the directory containing the native libraries and download the required files:

   - **Set the environment variable `ASPOSE_PDF_LIB_DIR`** to point to the folder where you will place the native `.bz2` archives, their `.sha256` checksum files, and the extracted native libraries (`.dll`, `.so`, `.dylib`, and for Windows also `.lib`):

     - On Linux/macOS:
       ```bash
       export ASPOSE_PDF_LIB_DIR=/path/to/lib
       ```

     - On Windows (Command Prompt):
       ```cmd
       set ASPOSE_PDF_LIB_DIR=C:\path\to\lib
       ```

     - On Windows (PowerShell):
       ```powershell
       $env:ASPOSE_PDF_LIB_DIR = "C:\path\to\lib"
       ```

   - **Download the required `.bz2` archives** and checksum files from the GitHub repository's [`lib/` folder](https://github.com/aspose-pdf/aspose-pdf-rust-cpp/tree/main/lib) and **place them** into the folder set in `ASPOSE_PDF_LIB_DIR`:

     - For **Linux x64**, download:
       - `libAsposePDFforRust_linux_amd64.so.bz2`
       - `libAsposePDFforRust_linux_amd64.so.bz2.sha256`

     - For **macOS x86_64**, download:
       - `libAsposePDFforRust_darwin_amd64.dylib.bz2`
       - `libAsposePDFforRust_darwin_amd64.dylib.bz2.sha256`

     - For **macOS arm64**, download:
       - `libAsposePDFforRust_darwin_arm64.dylib.bz2`
       - `libAsposePDFforRust_darwin_arm64.dylib.bz2.sha256`

     - For **Windows x64**, download:
       - `AsposePDFforRust_windows_amd64.dll.bz2`
       - `AsposePDFforRust_windows_amd64.dll.bz2.sha256`
       - `AsposePDFforRust_windows_amd64.lib` (native import library, not compressed)

		> **Note:** You need to manually download these files from GitHub and place them into the directory pointed by `ASPOSE_PDF_LIB_DIR`.  
		> The build script will automatically unpack the native libraries from the `.bz2` archives on first build.

3. **Build** your project (`cargo build`). On the first build, the native library matching your platform will be automatically extracted and linked. This step may take a few seconds.

> **Notes**
> - You must provide the folder containing the `.bz2` and `.sha256` files separately, as these binary archives are not distributed via crates.io.
> - If the required archive is missing or the checksum fails, the build will fail with a detailed error.
> - The same binary files used for installation via GitHub or the Aspose website can be reused here.

## Quick Start
All code snippets are contained in the [examples](./examples) folder.

### Hello World!

```rust
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
```

### Save PDF as Office Formats

One of the most popular features of Aspose.PDF for Rust via C++ is to convert PDF documents to other formats without needing to understand the underlying structure of the resultant format.

Give the following snippet a try with your samples:

```rust
use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Convert and save the previously opened PDF-document as DocX-document
    pdf.save_docx("sample.docx")?;

    Ok(())
}
```
### Extract Text From Whole PDF

```rust
use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Return the PDF-document contents as plain text
    let txt = pdf.extract_text()?;

    // Print extracted text
    println!("Extracted text:\n{}", txt);

    Ok(())
}
```

## Testing

Run the tests from the root of the package directory:

```sh
cargo test
```

## License

- The **Rust source code** is licensed under the [MIT License](LICENSE).
- The **shared library (`AsposePDFforRust_windows_amd64.dll`, `libAsposePDFforRust_linux_amd64.so`, `libAsposePDFforRust_darwin_amd64.dylib`, `libAsposePDFforRust_darwin_arm64.dylib`)** is proprietary and requires a commercial license.
  To use the full functionality, you must obtain a license.

### Evaluation version

You can use Aspose.PDF for Rust via C++ free of cost for evaluation.The evaluation version provides almost all functionality of the product with certain limitations. The same evaluation version becomes licensed when you purchase a license and add a couple of lines of code to apply the license.

>If you want to test Aspose.PDF for Rust without the evaluation version limitations, you can also request a 30-day Temporary License. Please refer to [How to get a Temporary License?](https://purchase.aspose.com/temporary-license)

### Limitation of an evaluation version

We want our customers to test our components thoroughly before buying so the evaluation version allows you to use it as you would normally.

- **Documents created with an evaluation watermark.** The evaluation version of Aspose.PDF for Rust provides full product functionality, but all pages in the generated files are watermarked with the text "Evaluation Only. Created with Aspose.PDF. Copyright 2002-2025 Aspose Pty Ltd." at the top.
- **Limit the number of pages that can be processed.** In the evaluation version, you can only process the first four pages of a document.

### Use in production

A commercial license key is required in a production environment. Please contact us to <a href="https://purchase.aspose.com/buy">purchase a commercial license</a>.

### Apply license

Applying a license to enable full functionality of the Aspose.PDF for Rust using a license file (Aspose.PDF.RustViaCPP.lic).

```rust
use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Set license with filename
    pdf.set_license("Aspose.PDF.RustViaCPP.lic")?;

    // Now you can work with the licensed PDF document
    // ...

    Ok(())
}
```

[Home](https://www.aspose.com/) | [Product Page](https://products.aspose.com/pdf/rust-cpp/) | [Docs](https://docs.aspose.com/pdf/rust-cpp/) | [GitHub](https://github.com/aspose-pdf/aspose-pdf-rust-cpp) | [Examples](https://github.com/aspose-pdf/aspose-pdf-rust-cpp/tree/main/examples) | [API Reference](https://reference.aspose.com/pdf/rust-cpp/) | [Blog](https://blog.aspose.com/category/pdf/) | [Search](https://search.aspose.com/) | [Free Support](https://forum.aspose.com/c/pdf) |  [Temporary License](https://purchase.aspose.com/temporary-license)
