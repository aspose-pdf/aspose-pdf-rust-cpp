#![allow(unsafe_code)]
use serde_json;
use std::ffi::{c_char, c_void, CStr, CString};

pub use super::enums::*;
pub use super::errors::*;
use super::extern_c::*;
pub use super::product_info::*;

use crate::debug_println;
use crate::generate_fn;

#[derive(Debug)]
pub struct Document {
    pdfdocumentclass: *const c_void,
}

impl Document {
    /// Create a new PDF-document.
    ///
    /// # Returns
    /// Returns `Ok(Self)` with a new PDF-document instance, or `Err(PdfError)` if creation fails.
    pub fn new() -> Result<Self, PdfError> {
        debug_println!("call Document::new()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let doctmp = Document {
            pdfdocumentclass: unsafe { PDFDocument_New(error.as_mut_ptr()) },
        };
        let error_str = Self::get_error(&mut error);
        if doctmp.pdfdocumentclass.is_null() {
            debug_println!("error Document::new(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        } else {
            Ok(doctmp)
        }
    }

    /// Open a PDF-document with filename.
    ///
    /// # Arguments
    /// * `filename` - Path to the PDF-document to open.
    ///
    /// # Returns
    /// Returns `Ok(Self)` with the opened PDF-document instance, or `Err(PdfError)` if opening fails.
    pub fn open(filename: &str) -> Result<Self, PdfError> {
        debug_println!("call Document::open({filename:?})");
        let filename_c_string = CString::new(filename).unwrap();
        let filename_c_char_ptr: *const c_char = filename_c_string.as_ptr();
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let doctmp = Document {
            pdfdocumentclass: unsafe { PDFDocument_Open(filename_c_char_ptr, error.as_mut_ptr()) },
        };
        let error_str = Self::get_error(&mut error);
        if doctmp.pdfdocumentclass.is_null() {
            debug_println!("error Document::open({filename}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        } else {
            Ok(doctmp)
        }
    }

    /// Returns metadata information about the Aspose.PDF for Rust via C++.
    ///
    /// The metadata is returned as a `ProductInfo` struct, deserialized from a JSON string.
    /// It includes product name, version, release date, licensing status, and related details.
    ///
    /// See also: `product_info.rs`
    ///
    /// # Returns
    /// Returns `Ok(ProductInfo)` containing product metadata, or `Err(PdfError)` if failed.
    pub fn about(&self) -> Result<ProductInfo, PdfError> {
        debug_println!("call Document::about()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let char_ptr = unsafe { PDFDocument_About(self.pdfdocumentclass, error.as_mut_ptr()) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        let json_str = c_str.to_str().map(|s| s.to_owned()).unwrap();
        unsafe { c_free_string(char_ptr as *mut c_char) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            let info: ProductInfo = serde_json::from_str(&json_str)
                .map_err(|e| PdfError::CoreExceptionError(e.to_string()))?;
            Ok(info)
        } else {
            debug_println!("error Document::about(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return the PDF-document contents as plain text.
    ///
    /// # Returns
    /// Returns `Ok(String)` containing the extracted text, or `Err(PdfError)` if extraction fails.
    pub fn extract_text(&self) -> Result<String, PdfError> {
        debug_println!("call Document::extract_text()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let char_ptr =
            unsafe { PDFDocument_ExtractText(self.pdfdocumentclass, error.as_mut_ptr()) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        let text = c_str.to_str().map(|s| s.to_owned()).unwrap();
        unsafe { c_free_string(char_ptr as *mut c_char) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(text)
        } else {
            debug_println!("error Document::extract_text(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Set PDF-document background color using RGB values.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255).
    /// * `g` - Green component (0-255).
    /// * `b` - Blue component (0-255).
    ///
    /// # Errors
    /// Returns `PdfError` if setting the background color fails.
    pub fn set_background(&self, r: i32, g: i32, b: i32) -> Result<(), PdfError> {
        debug_println!("call Document::set_background({r:?}, {g:?}, {b:?})");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe { PDFDocument_set_Background(self.pdfdocumentclass, r, g, b, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::set_background({r:?}, {g:?}, {b:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Rotate PDF-document.
    ///
    /// # Arguments
    /// * `rotation` - Rotation angle as enum `Rotation`:
    ///   `None`, `On90`, `On180`, `On270`, or `On360`.
    ///
    /// # Errors
    /// Returns `PdfError` if the rotation operation fails.
    pub fn rotate(&self, rotation: Rotation) -> Result<(), PdfError> {
        debug_println!("call Document::rotate({rotation:?})");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Rotate(
                self.pdfdocumentclass,
                rotation.clone() as i32,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::rotate({rotation:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Rotate a page in the PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `rotation` - Rotation angle as enum `Rotation`:
    ///   `None`, `On90`, `On180`, `On270`, or `On360`.
    ///
    /// # Errors
    /// Returns `PdfError` if the rotation operation fails.
    pub fn page_rotate(&self, num: i32, rotation: Rotation) -> Result<(), PdfError> {
        debug_println!("call Document::page_rotate({rotation:?})");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Page_Rotate(
                self.pdfdocumentclass,
                num,
                rotation.clone() as i32,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::page_rotate({rotation:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Set the size of a page in the PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `page_size` - Page size as enum `PageSize`:
    ///   `A0`, `A1`, `A2`, `A3`, `A4`, `A5`, `A6`, `B5`, `PageLetter`, `PageLegal`, `PageLedger`, or `P11x17`.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_set_size(&self, num: i32, page_size: PageSize) -> Result<(), PdfError> {
        debug_println!("call Document::page_set_size({page_size:?})");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Page_set_Size(
                self.pdfdocumentclass,
                num,
                page_size.clone() as i32,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::page_set_size({page_size:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return the number of pages in the PDF-document.
    ///
    /// # Returns
    /// * `Ok(i32)` - The total number of pages.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn page_count(&self) -> Result<i32, PdfError> {
        debug_println!("call Document::page_count()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let page_count: i32 =
            unsafe { PDFDocument_Page_get_Count(self.pdfdocumentclass, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(page_count)
        } else {
            debug_println!("error Document::page_count(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    // Converts raw error pointer from FFI into Rust String.
    // Used internally to check and propagate C++ errors as PdfError.
    fn get_error(error: &mut std::mem::MaybeUninit<*const c_char>) -> String {
        let error_assume_init = unsafe { error.assume_init() as *mut c_char };
        let error_c_str: &CStr = unsafe { CStr::from_ptr(error_assume_init) };
        let error_str_tmp = error_c_str.to_string_lossy();
        let error_str = String::from(error_str_tmp); //            let error_str = error_str_tmp.to_string();
        unsafe { c_free_string(error_assume_init) };
        error_str
    }

    // Generated functions
    generate_fn!(_save, PDFDocument_Save);
    generate_fn!(_save_as, PDFDocument_Save_As, filename: &str);
    generate_fn!(_set_license, PDFDocument_set_License, filename: &str);

    generate_fn!(_save_docx_enhanced, PDFDocument_Save_DocXEnhanced, filename: &str);
    generate_fn!(_save_docx, PDFDocument_Save_DocX, filename: &str);
    generate_fn!(_save_doc, PDFDocument_Save_Doc, filename: &str);
    generate_fn!(_save_xlsx, PDFDocument_Save_XlsX, filename: &str);
    generate_fn!(_save_pptx, PDFDocument_Save_PptX, filename: &str);
    generate_fn!(_save_xps, PDFDocument_Save_Xps, filename: &str);
    generate_fn!(_save_txt, PDFDocument_Save_Txt, filename: &str);
    generate_fn!(_save_epub, PDFDocument_Save_Epub, filename: &str);
    generate_fn!(_save_tex, PDFDocument_Save_TeX, filename: &str);
    generate_fn!(_save_markdown, PDFDocument_Save_Markdown, filename: &str);
    generate_fn!(_save_booklet, PDFDocument_Save_Booklet, filename: &str);
    generate_fn!(_save_n_up, PDFDocument_Save_NUp, filename: &str, columns: i32, rows: i32);
    generate_fn!(_save_tiff, PDFDocument_Save_Tiff, resolution_dpi: i32, filename: &str);
    generate_fn!(_save_svg_zip, PDFDocument_Save_SvgZip, filename: &str);
    generate_fn!(_export_fdf, PDFDocument_Export_Fdf, filename: &str);
    generate_fn!(_export_xfdf, PDFDocument_Export_Xfdf, filename: &str);
    generate_fn!(_export_xml, PDFDocument_Export_Xml, filename: &str);

    generate_fn!(_optimize, PDFDocument_Optimize);
    generate_fn!(_optimize_resource, PDFDocument_OptimizeResource);
    generate_fn!(_repair, PDFDocument_Repair);
    generate_fn!(_grayscale, PDFDocument_Grayscale);

    generate_fn!(_replace_text, PDFDocument_ReplaceText, find_text: &str, replace_text: &str);
    generate_fn!(_add_page_num, PDFDocument_AddPageNum);
    generate_fn!(_add_text_header, PDFDocument_AddTextHeader, header: &str);
    generate_fn!(_add_text_footer, PDFDocument_AddTextFooter, footer: &str);

    generate_fn!(_flatten, PDFDocument_Flatten);
    generate_fn!(_remove_annotations, PDFDocument_RemoveAnnotations);
    generate_fn!(_remove_attachments, PDFDocument_RemoveAttachments);
    generate_fn!(_remove_blank_pages, PDFDocument_RemoveBlankPages);
    generate_fn!(_remove_bookmarks, PDFDocument_RemoveBookmarks);
    generate_fn!(_remove_hidden_text, PDFDocument_RemoveHiddenText);
    generate_fn!(_remove_images, PDFDocument_RemoveImages);
    generate_fn!(_remove_javascripts, PDFDocument_RemoveJavaScripts);

    generate_fn!(_page_to_jpg, PDFDocument_Page_to_Jpg, num: i32, resolution_dpi: i32, filename: &str);
    generate_fn!(_page_to_png, PDFDocument_Page_to_Png, num: i32, resolution_dpi: i32, filename: &str);
    generate_fn!(_page_to_bmp, PDFDocument_Page_to_Bmp, num: i32, resolution_dpi: i32, filename: &str);
    generate_fn!(_page_to_tiff, PDFDocument_Page_to_Tiff, num: i32, resolution_dpi: i32, filename: &str);
    generate_fn!(_page_to_svg, PDFDocument_Page_to_Svg, num: i32, filename: &str);
    generate_fn!(_page_to_pdf, PDFDocument_Page_to_Pdf, num: i32, filename: &str);
    generate_fn!(_page_to_dicom, PDFDocument_Page_to_DICOM, num: i32, resolution_dpi: i32, filename: &str);

    generate_fn!(_page_add, PDFDocument_Page_Add);
    generate_fn!(_page_insert, PDFDocument_Page_Insert, num: i32);
    generate_fn!(_page_delete, PDFDocument_Page_Delete, num: i32);
    generate_fn!(_page_grayscale, PDFDocument_Page_Grayscale, num: i32);
    generate_fn!(_page_add_text, PDFDocument_Page_AddText, num: i32, add_text: &str);

    generate_fn!(_page_replace_text, PDFDocument_Page_ReplaceText, num: i32, find_text: &str, replace_text: &str);
    generate_fn!(_page_add_page_num, PDFDocument_Page_AddPageNum, num: i32);
    generate_fn!(_page_add_text_header, PDFDocument_Page_AddTextHeader, num: i32, header: &str);
    generate_fn!(_page_add_text_footer, PDFDocument_Page_AddTextFooter, num: i32, footer: &str);

    generate_fn!(_page_remove_annotations, PDFDocument_Page_RemoveAnnotations, num: i32);
    generate_fn!(_page_remove_hidden_text, PDFDocument_Page_RemoveHiddenText, num: i32);
    generate_fn!(_page_remove_images, PDFDocument_Page_RemoveImages, num: i32);

    /// Save the previously opened PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save(&self) -> Result<(), PdfError> {
        self._save()
    }

    /// Save the previously opened PDF-document with new filename.
    ///
    /// # Arguments
    /// * `filename` - The new path to the PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_as(&self, filename: &str) -> Result<(), PdfError> {
        self._save_as(filename)
    }

    /// Set license with filename.
    ///
    /// # Arguments
    /// * `filename` - The path to the license-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn set_license(&self, filename: &str) -> Result<(), PdfError> {
        self._set_license(filename)
    }

    /// Convert and save the previously opened PDF-document as DocX-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output DOCX-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_docx(&self, filename: &str) -> Result<(), PdfError> {
        self._save_docx(filename)
    }

    /// Convert and save the previously opened PDF-document as DocX-document with Enhanced Recognition Mode (fully editable tables and paragraphs).
    ///
    /// # Arguments
    /// * `filename` - The path to the output DOCX-file with Enhanced Recognition Mode.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_docx_enhanced(&self, filename: &str) -> Result<(), PdfError> {
        self._save_docx_enhanced(filename)
    }

    /// Convert and save the previously opened PDF-document as Doc-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output DOC-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_doc(&self, filename: &str) -> Result<(), PdfError> {
        self._save_doc(filename)
    }

    /// Convert and save the previously opened PDF-document as XlsX-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output XLSX-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_xlsx(&self, filename: &str) -> Result<(), PdfError> {
        self._save_xlsx(filename)
    }

    /// Convert and save the previously opened PDF-document as PptX-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output PPTX-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_pptx(&self, filename: &str) -> Result<(), PdfError> {
        self._save_pptx(filename)
    }

    /// Convert and save the previously opened PDF-document as Xps-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output XPS-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_xps(&self, filename: &str) -> Result<(), PdfError> {
        self._save_xps(filename)
    }

    /// Convert and save the previously opened PDF-document as Txt-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output TXT-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_txt(&self, filename: &str) -> Result<(), PdfError> {
        self._save_txt(filename)
    }

    /// Convert and save the previously opened PDF-document as Epub-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output EPUB-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_epub(&self, filename: &str) -> Result<(), PdfError> {
        self._save_epub(filename)
    }

    /// Convert and save the previously opened PDF-document as TeX-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output TEX-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_tex(&self, filename: &str) -> Result<(), PdfError> {
        self._save_tex(filename)
    }

    /// Convert and save the previously opened PDF-document as Markdown-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output MARKDOWN-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_markdown(&self, filename: &str) -> Result<(), PdfError> {
        self._save_markdown(filename)
    }

    /// Convert and save the previously opened PDF-document as booklet PDF-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output booklet PDF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_booklet(&self, filename: &str) -> Result<(), PdfError> {
        self._save_booklet(filename)
    }

    /// Convert and save the previously opened PDF-document as N-Up PDF-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output N-Up PDF-file.
    /// * `columns` - The number of columns.
    /// * `rows` - The number of rows.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_n_up(&self, filename: &str, columns: i32, rows: i32) -> Result<(), PdfError> {
        self._save_n_up(filename, columns, rows)
    }

    /// Convert and save the previously opened PDF-document as TIFF-document.
    ///
    /// # Arguments
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the output TIFF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_tiff(&self, resolution_dpi: i32, filename: &str) -> Result<(), PdfError> {
        self._save_tiff(resolution_dpi, filename)
    }

    /// Convert and save the previously opened PDF-document as SVG-archive.
    ///
    /// # Arguments
    /// * `filename` - The path to the output SVG-archive.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn save_svg_zip(&self, filename: &str) -> Result<(), PdfError> {
        self._save_svg_zip(filename)
    }

    /// Export from the previously opened PDF-document with AcroForm to FDF-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output FDF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn export_fdf(&self, filename: &str) -> Result<(), PdfError> {
        self._export_fdf(filename)
    }

    /// Export from the previously opened PDF-document with AcroForm to XFDF-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output XFDF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn export_xfdf(&self, filename: &str) -> Result<(), PdfError> {
        self._export_xfdf(filename)
    }

    /// Export from the previously opened PDF-document with AcroForm to XML-document.
    ///
    /// # Arguments
    /// * `filename` - The path to the output XML-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn export_xml(&self, filename: &str) -> Result<(), PdfError> {
        self._export_xml(filename)
    }

    /// Optimize PDF-document content.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn optimize(&self) -> Result<(), PdfError> {
        self._optimize()
    }

    /// Optimize resources of PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn optimize_resource(&self) -> Result<(), PdfError> {
        self._optimize_resource()
    }

    /// Repair PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn repair(&self) -> Result<(), PdfError> {
        self._repair()
    }

    /// Convert PDF-document to black and white.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn grayscale(&self) -> Result<(), PdfError> {
        self._grayscale()
    }

    /// Replace text in PDF-document.
    ///
    /// # Arguments
    /// * `find_text` - The text fragment to search.
    /// * `replace_text` - The text fragment to replace.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn replace_text(&self, find_text: &str, replace_text: &str) -> Result<(), PdfError> {
        self._replace_text(find_text, replace_text)
    }

    /// Add page number to a PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn add_page_num(&self) -> Result<(), PdfError> {
        self._add_page_num()
    }

    /// Add text in Header of a PDF-document.
    ///
    /// # Arguments
    /// * `header` - The pages header.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn add_text_header(&self, header: &str) -> Result<(), PdfError> {
        self._add_text_header(header)
    }

    /// Add text in Footer of a PDF-document.
    ///
    /// # Arguments
    /// * `footer` - The pages footer.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn add_text_footer(&self, footer: &str) -> Result<(), PdfError> {
        self._add_text_footer(footer)
    }

    /// Flatten PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn flatten(&self) -> Result<(), PdfError> {
        self._flatten()
    }

    /// Remove annotations from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_annotations(&self) -> Result<(), PdfError> {
        self._remove_annotations()
    }

    /// Remove attachments from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_attachments(&self) -> Result<(), PdfError> {
        self._remove_attachments()
    }

    /// Remove blank pages from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_blank_pages(&self) -> Result<(), PdfError> {
        self._remove_blank_pages()
    }

    /// Remove bookmarks from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_bookmarks(&self) -> Result<(), PdfError> {
        self._remove_bookmarks()
    }

    /// Remove hidden text from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_hidden_text(&self) -> Result<(), PdfError> {
        self._remove_hidden_text()
    }

    /// Remove images from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_images(&self) -> Result<(), PdfError> {
        self._remove_images()
    }

    /// Remove java scripts from PDF-document
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_javascripts(&self) -> Result<(), PdfError> {
        self._remove_javascripts()
    }

    /// Convert and save the specified page as Jpg-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the JPG-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_jpg(
        &self,
        num: i32,
        resolution_dpi: i32,
        filename: &str,
    ) -> Result<(), PdfError> {
        self._page_to_jpg(num, resolution_dpi, filename)
    }

    /// Convert and save the specified page as Png-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the PNG-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_png(
        &self,
        num: i32,
        resolution_dpi: i32,
        filename: &str,
    ) -> Result<(), PdfError> {
        self._page_to_png(num, resolution_dpi, filename)
    }

    /// Convert and save the specified page as Bmp-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the BMP-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_bmp(
        &self,
        num: i32,
        resolution_dpi: i32,
        filename: &str,
    ) -> Result<(), PdfError> {
        self._page_to_bmp(num, resolution_dpi, filename)
    }

    /// Convert and save the specified page as Tiff-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the TIFF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_tiff(
        &self,
        num: i32,
        resolution_dpi: i32,
        filename: &str,
    ) -> Result<(), PdfError> {
        self._page_to_tiff(num, resolution_dpi, filename)
    }

    /// Convert and save the specified page as Svg-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `filename` - The path to the SVG-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_svg(&self, num: i32, filename: &str) -> Result<(), PdfError> {
        self._page_to_svg(num, filename)
    }

    /// Convert and save the specified page as Pdf.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `filename` - The path to the PDF-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_pdf(&self, num: i32, filename: &str) -> Result<(), PdfError> {
        self._page_to_pdf(num, filename)
    }

    /// Convert and save the specified page as DICOM-image.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `resolution_dpi` - The resolution in DPI.
    /// * `filename` - The path to the DICOM-file.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_to_dicom(
        &self,
        num: i32,
        resolution_dpi: i32,
        filename: &str,
    ) -> Result<(), PdfError> {
        self._page_to_dicom(num, resolution_dpi, filename)
    }

    /// Add new page in PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_add(&self) -> Result<(), PdfError> {
        self._page_add()
    }

    /// Insert new page at the specified position in PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page index (1-based) to insert at.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_insert(&self, num: i32) -> Result<(), PdfError> {
        self._page_insert(num)
    }

    /// Delete specified page in PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based) to delete.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_delete(&self, num: i32) -> Result<(), PdfError> {
        self._page_delete(num)
    }

    /// Convert page to black and white.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_grayscale(&self, num: i32) -> Result<(), PdfError> {
        self._page_grayscale(num)
    }

    /// Add text on page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `add_text` - The text to add.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_add_text(&self, num: i32, add_text: &str) -> Result<(), PdfError> {
        self._page_add_text(num, add_text)
    }

    /// Replace text on page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `find_text` - The text fragment to search.
    /// * `replace_text` - The text fragment to replace.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_replace_text(
        &self,
        num: i32,
        find_text: &str,
        replace_text: &str,
    ) -> Result<(), PdfError> {
        self._page_replace_text(num, find_text, replace_text)
    }

    /// Add page number on page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_add_page_num(&self, num: i32) -> Result<(), PdfError> {
        self._page_add_page_num(num)
    }

    /// Add text in page header.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `header` - The pages header.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_add_text_header(&self, num: i32, header: &str) -> Result<(), PdfError> {
        self._page_add_text_header(num, header)
    }

    /// Add text in page footer.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `footer` - The pages footer.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_add_text_footer(&self, num: i32, footer: &str) -> Result<(), PdfError> {
        self._page_add_text_footer(num, footer)
    }

    /// Remove annotations in page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_remove_annotations(&self, num: i32) -> Result<(), PdfError> {
        self._page_remove_annotations(num)
    }

    /// Remove hidden text in page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_remove_hidden_text(&self, num: i32) -> Result<(), PdfError> {
        self._page_remove_hidden_text(num)
    }

    /// Remove images in page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_remove_images(&self, num: i32) -> Result<(), PdfError> {
        self._page_remove_images(num)
    }
}

// Automatically called when the `Document` instance goes out of scope.
// Ensures the underlying C++ PDFDocument object is properly released.
impl Drop for Document {
    fn drop(&mut self) {
        debug_println!("call Document::drop()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe { PDFDocument_Release(self.pdfdocumentclass, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if !error_str.is_empty() {
            debug_println!("error Document::drop(): {error_str:?}");
        }
    }
}

// Provides a default implementation for `Document` using `Document::new()`.
// Panics if creation fails, which is acceptable for `Default` in this context.
impl Default for Document {
    fn default() -> Self {
        match Document::new() {
            Ok(document) => document,
            Err(error) => panic!("panic Document::default(): {error:?}"),
        }
    }
}
