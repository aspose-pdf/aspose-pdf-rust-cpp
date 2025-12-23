#![allow(unsafe_code)]
use serde_json;
use std::ffi::{c_char, c_uchar, c_void, CStr, CString};

use crate::enums::{CryptoAlgorithm, PageSize, Rotation};
use crate::errors::PdfError;
use crate::extern_c::*;
use crate::permissions::Permissions;
use crate::product_info::ProductInfo;

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

    /// Open a password-protected PDF-document.
    ///
    /// # Arguments
    /// * `filename` - Path to the PDF-document to open.
    /// * `password` - User/owner password of the password-protected PDF-document.
    ///
    /// # Returns
    /// Returns `Ok(Self)` with the opened PDF-document instance, or `Err(PdfError)` if opening fails.
    pub fn open_with_password(filename: &str, password: &str) -> Result<Self, PdfError> {
        debug_println!("call Document::open_with_password({filename:?})");
        let filename_c_string = CString::new(filename).unwrap();
        let filename_c_char_ptr: *const c_char = filename_c_string.as_ptr();
        let password_c_string = CString::new(password).unwrap();
        let password_c_char_ptr: *const c_char = password_c_string.as_ptr();
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let doctmp = Document {
            pdfdocumentclass: unsafe {
                PDFDocument_Open_With_Password(
                    filename_c_char_ptr,
                    password_c_char_ptr,
                    error.as_mut_ptr(),
                )
            },
        };
        let error_str = Self::get_error(&mut error);
        if doctmp.pdfdocumentclass.is_null() {
            debug_println!("error Document::open_with_password({filename}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        } else {
            Ok(doctmp)
        }
    }

    /// Create a new PDF-document by merging the provided PDF-documents.
    ///
    /// # Arguments
    /// * `documents` - A slice of references to PDF-documents [`Document`] to merge.
    ///
    /// # Returns
    /// Returns `Ok(Self)` with a new PDF-document instance, or `Err(PdfError)` if creation fails.
    pub fn merge_documents(documents: &[&Document]) -> Result<Self, PdfError> {
        debug_println!(
            "call Document::merge_documents(), total docs: {}",
            documents.len()
        );

        if documents.is_empty() {
            return Err(PdfError::CoreExceptionError(
                "merge_documents(): no documents to merge".to_string(),
            ));
        }

        let merged = Self::new()?;

        for (i, doc) in documents.iter().enumerate() {
            if doc.pdfdocumentclass.is_null() {
                return Err(PdfError::CoreExceptionError(format!(
                    "merge_documents(): document at index {} is null",
                    i
                )));
            }

            if let Err(e) = merged.append(doc) {
                return Err(PdfError::CoreExceptionError(format!(
                    "merge_documents(): failed to append document at index {}: {}",
                    i, e
                )));
            }
        }

        Ok(merged)
    }

    // Helper function used by `split` and `split_document`.
    // Splits the source document into multiple documents based on the page range string.
    fn _split_document(document: &Document, page_range: &str) -> Result<Vec<Self>, PdfError> {
        if document.pdfdocumentclass.is_null() {
            return Err(PdfError::CoreExceptionError(
                "split_document(): source document is null".to_string(),
            ));
        }

        if page_range.trim().is_empty() {
            return Err(PdfError::CoreExceptionError(
                "split_document(): empty page range string".to_string(),
            ));
        }

        let mut result = Vec::new();

        for (i, part) in page_range.split(';').enumerate() {
            let part = part.trim();
            if part.is_empty() {
                return Err(PdfError::CoreExceptionError(format!(
                    "split_document(): empty page range at index {}",
                    i
                )));
            }

            let new_doc = Self::new().map_err(|e| {
                PdfError::CoreExceptionError(format!(
                    "split_document(): failed to create new document for range \"{}\": {}",
                    part, e
                ))
            })?;

            new_doc.append_pages(document, part).map_err(|e| {
                PdfError::CoreExceptionError(format!(
                    "split_document(): failed to append pages \"{}\": {}",
                    part, e
                ))
            })?;

            result.push(new_doc);
        }

        Ok(result)
    }
    /// Create multiple new PDF-documents by extracting pages from the source PDF-document.
    ///
    /// Each part of the `page_range` string (separated by `;`) defines the page range for a new PDF-document.
    /// For example, `"1-2;3;4-"` will produce three documents: pages 1–2, page 3, and pages 4 to end.
    ///
    /// # Arguments
    /// * `document` - A reference to the source PDF-document [`Document`] to split.
    /// * `page_range` - A string [`&str`] specifying page ranges, e.g. `"1-2;3;4-"`.
    ///
    /// # Returns
    /// Returns `Ok(Vec<Self>)` containing the resulting split documents,
    /// or `Err(PdfError)` if splitting fails.
    pub fn split_document(document: &Document, page_range: &str) -> Result<Vec<Self>, PdfError> {
        debug_println!(
            "call Document::split_document(), page_range: \"{}\"",
            page_range
        );
        Document::_split_document(document, page_range)
    }

    // Helper function used by `split_at` and `split_at_page`.
    // Splits the source document into two parts: [1..=page] and [page+1..end].
    fn _split_at_page(document: &Document, page: i32) -> Result<(Self, Self), PdfError> {
        if document.pdfdocumentclass.is_null() {
            return Err(PdfError::CoreExceptionError(
                "split_at_page: document is null".to_string(),
            ));
        }

        let page_count = document.page_count()?;
        if page < 1 || page >= page_count {
            return Err(PdfError::CoreExceptionError(format!(
                "split_at_page: page {} is out of valid range (1-{} exclusive)",
                page, page_count
            )));
        }

        // Create the left document (pages 1 to `page`)
        let left = Document::new().map_err(|e| {
            PdfError::CoreExceptionError(format!(
                "split_at_page: failed to create first document: {}",
                e
            ))
        })?;

        let left_range = format!("1-{}", page);
        left.append_pages(document, &left_range).map_err(|e| {
            PdfError::CoreExceptionError(format!(
                "split_at_page: failed to append left pages '{}': {}",
                left_range, e
            ))
        })?;

        // Create the right document (pages `page + 1` to end)
        let right = Document::new().map_err(|e| {
            PdfError::CoreExceptionError(format!(
                "split_at_page: failed to create second document: {}",
                e
            ))
        })?;

        let right_range = format!("{}-", page + 1);
        right.append_pages(document, &right_range).map_err(|e| {
            PdfError::CoreExceptionError(format!(
                "split_at_page: failed to append right pages '{}': {}",
                right_range, e
            ))
        })?;

        Ok((left, right))
    }

    /// Split the PDF-document into two new PDF-documents.
    ///
    /// The first document includes pages 1 to `page` (inclusive).
    /// The second document includes pages from `page + 1` to the end.
    ///
    /// # Arguments
    /// * `document` - A reference to the source PDF-document [`Document`] to split.
    /// * `page` - The page number at which to split (1-based, exclusive for the second part).
    ///
    /// # Returns
    /// Returns `Ok((Self, Self))` with the two split documents,
    /// or `Err(PdfError)` if the operation fails.
    pub fn split_at_page(document: &Document, page: i32) -> Result<(Self, Self), PdfError> {
        debug_println!("call Document::split_at_page(page = {})", page);
        Document::_split_at_page(document, page)
    }

    /// Return metadata information about the Aspose.PDF for Rust via C++.
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

    /// Append pages from another PDF-document.
    ///
    /// # Arguments
    /// * `other` - A reference to another PDF-document [`Document`] to append pages from.
    ///
    /// # Errors
    /// Returns `PdfError` if appending fails.
    pub fn append(&self, other: &Document) -> Result<(), PdfError> {
        debug_println!("call Document::append()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Append(
                self.pdfdocumentclass,
                other.pdfdocumentclass,
                error.as_mut_ptr(),
            );
        }
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::append(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Append selected pages from another PDF-document.
    ///
    /// # Arguments
    /// * `other` - A reference to another PDF-document [`Document`] to append pages from.
    /// * `page_range` - A string defining the page ranges to append (e.g. "-2,4,6-8,10-").
    ///
    /// # Errors
    /// Returns `PdfError` if appending fails.
    pub fn append_pages(&self, other: &Document, page_range: &str) -> Result<(), PdfError> {
        debug_println!("call Document::append_pages({page_range})");
        let c_page_range = CString::new(page_range).unwrap();
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_AppendPages(
                self.pdfdocumentclass,
                other.pdfdocumentclass,
                c_page_range.as_ptr(),
                error.as_mut_ptr(),
            );
        }
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::append_pages(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Create multiple new PDF-documents by extracting pages from the current PDF-document.
    ///
    /// Each part of the `page_range` string (separated by `;`) defines the page range for a new PDF-document.
    /// For example, `"1-2;3;4-"` will produce three documents: pages 1–2, page 3, and pages 4 to end.
    ///
    /// # Arguments
    /// * `page_range` - A string [`&str`] specifying page ranges, e.g. `"1-2;3;4-"`.
    ///
    /// # Returns
    /// Returns `Ok(Vec<Self>)` containing the resulting split documents,
    /// or `Err(PdfError)` if splitting fails.
    pub fn split(&self, page_range: &str) -> Result<Vec<Self>, PdfError> {
        debug_println!("call Document::split(), page_range: \"{}\"", page_range);
        Self::_split_document(self, page_range)
    }

    /// Split the current PDF-document into two new PDF-documents.
    ///
    /// The first document includes pages 1 to `page` (inclusive).
    /// The second document includes pages from `page + 1` to the end.
    ///
    /// # Arguments
    /// * `page` - The page number at which to split (1-based, exclusive for the second part).
    ///
    /// # Returns
    /// Returns `Ok((Self, Self))` with the two split documents,
    /// or `Err(PdfError)` if the operation fails.
    pub fn split_at(&self, page: i32) -> Result<(Self, Self), PdfError> {
        debug_println!("call Document::split_at(page = {})", page);
        Self::_split_at_page(self, page)
    }

    /// Return the contents of the PDF-document as a byte vector.
    ///
    /// # Returns
    /// Returns `Ok(Vec<u8>)` containing the binary contents, or `Err(PdfError)` if the operation fails.
    pub fn bytes(&self) -> Result<Vec<u8>, PdfError> {
        debug_println!("call Document::bytes()");

        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let mut buf: *mut c_uchar = std::ptr::null_mut();
        let mut size: i32 = 0;

        unsafe {
            PDFDocument_Save_Memory(
                self.pdfdocumentclass,
                &mut buf,
                &mut size,
                error.as_mut_ptr(),
            );
        }

        let error_str = Self::get_error(&mut error);

        if error_str.is_empty() && !buf.is_null() && size > 0 {
            // Copy the buffer into a Rust Vec<u8>
            let bytes = unsafe { std::slice::from_raw_parts(buf, size as usize).to_vec() };
            unsafe { c_free_buffer(buf.cast()) };
            Ok(bytes)
        } else {
            // Clean up even if there was an error
            if !buf.is_null() {
                unsafe { c_free_buffer(buf.cast()) };
            }

            debug_println!("error Document::bytes(): {error_str:?}");
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

    /// Return word count in PDF-document.
    ///
    /// # Returns
    /// * `Ok(i32)` - The word count.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn word_count(&self) -> Result<i32, PdfError> {
        debug_println!("call Document::word_count()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let word_count: i32 =
            unsafe { PDFDocument_get_WordCount(self.pdfdocumentclass, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(word_count)
        } else {
            debug_println!("error Document::word_count(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return character count in PDF-document.
    ///
    /// # Returns
    /// * `Ok(i32)` - The character count.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn character_count(&self) -> Result<i32, PdfError> {
        debug_println!("call Document::character_count()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let character_count: i32 =
            unsafe { PDFDocument_get_CharacterCount(self.pdfdocumentclass, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(character_count)
        } else {
            debug_println!("error Document::character_count(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return word count on specified page in PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Returns
    /// * `Ok(i32)` - The word count.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn page_word_count(&self, num: i32) -> Result<i32, PdfError> {
        debug_println!("call Document::page_word_count()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let page_word_count: i32 = unsafe {
            PDFDocument_Page_get_WordCount(self.pdfdocumentclass, num, error.as_mut_ptr())
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(page_word_count)
        } else {
            debug_println!("error Document::page_word_count(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return character count on specified page in PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Returns
    /// * `Ok(i32)` - The character count.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn page_character_count(&self, num: i32) -> Result<i32, PdfError> {
        debug_println!("call Document::page_character_count()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let page_character_count: i32 = unsafe {
            PDFDocument_Page_get_CharacterCount(self.pdfdocumentclass, num, error.as_mut_ptr())
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(page_character_count)
        } else {
            debug_println!("error Document::page_character_count(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Return page is blank in PDF-document.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Returns
    /// * `Ok(bool)` - True if the page is blank.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn page_is_blank(&self, num: i32) -> Result<bool, PdfError> {
        debug_println!("call Document::page_is_blank()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let page_is_blank: i32 =
            unsafe { PDFDocument_Page_is_Blank(self.pdfdocumentclass, num, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(page_is_blank != 0)
        } else {
            debug_println!("error Document::page_is_blank(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    // Converts raw error pointer from FFI into Rust String.
    // Used internally to check and propagate C++ errors as PdfError.
    fn get_error(error: &mut std::mem::MaybeUninit<*const c_char>) -> String {
        let error_assume_init = unsafe { error.assume_init() as *mut c_char };
        let error_str = unsafe {
            CStr::from_ptr(error_assume_init)
                .to_string_lossy()
                .into_owned()
        };
        unsafe { c_free_string(error_assume_init) };
        error_str
    }

    /// Add watermark to PDF-document.
    ///
    /// # Arguments
    /// * `text` - The watermark text.
    /// * `font_name` - The font name.
    /// * `font_size` - The font size.
    /// * `foreground_color` - The text color (hexadecimal format "#RRGGBB", where RR-red, GG-green and BB-blue hexadecimal integers).
    /// * `x_position` - The 'x' watermark position.
    /// * `y_position` - The 'y' watermark position.
    /// * `rotation` - The watermark rotation (0-360).
    /// * `is_background` - The background.
    /// * `opacity` - The opacity (decimal).
    ///
    /// # Errors
    /// Returns `PdfError` if the rotation operation fails.
    pub fn add_watermark(
        &self,
        text: &str,
        font_name: &str,
        font_size: f64,
        foreground_color: &str,
        x_position: i32,
        y_position: i32,
        rotation: i32,
        is_background: bool,
        opacity: f64,
    ) -> Result<(), PdfError> {
        debug_println!("call Document::add_watermark({text:?})");
        let c_string_text = std::ffi::CString::new(text).unwrap();
        let c_char_ptr_text = c_string_text.as_ptr();
        let c_string_font_name = std::ffi::CString::new(font_name).unwrap();
        let c_char_ptr_font_name = c_string_font_name.as_ptr();
        let c_string_foreground_color = std::ffi::CString::new(foreground_color).unwrap();
        let c_char_ptr_foreground_color = c_string_foreground_color.as_ptr();
        let _is_background: i32 = if is_background { 1 } else { 0 };
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_AddWatermark(
                self.pdfdocumentclass,
                c_char_ptr_text as *const c_char,
                c_char_ptr_font_name as *const c_char,
                font_size,
                c_char_ptr_foreground_color as *const c_char,
                x_position,
                y_position,
                rotation,
                _is_background,
                opacity,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::add_watermark({text:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Add watermark on page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    /// * `text` - The watermark text.
    /// * `font_name` - The font name.
    /// * `font_size` - The font size.
    /// * `foreground_color` - The text color (hexadecimal format "#RRGGBB", where RR-red, GG-green and BB-blue hexadecimal integers).
    /// * `x_position` - The 'x' watermark position.
    /// * `y_position` - The 'y' watermark position.
    /// * `rotation` - The watermark rotation (0-360).
    /// * `is_background` - The background.
    /// * `opacity` - The opacity (decimal).
    ///
    /// # Errors
    /// Returns `PdfError` if the rotation operation fails.
    pub fn page_add_watermark(
        &self,
        num: i32,
        text: &str,
        font_name: &str,
        font_size: f64,
        foreground_color: &str,
        x_position: i32,
        y_position: i32,
        rotation: i32,
        is_background: bool,
        opacity: f64,
    ) -> Result<(), PdfError> {
        debug_println!("call Document::page_add_watermark({text:?})");
        let c_string_text = std::ffi::CString::new(text).unwrap();
        let c_char_ptr_text = c_string_text.as_ptr();
        let c_string_font_name = std::ffi::CString::new(font_name).unwrap();
        let c_char_ptr_font_name = c_string_font_name.as_ptr();
        let c_string_foreground_color = std::ffi::CString::new(foreground_color).unwrap();
        let c_char_ptr_foreground_color = c_string_foreground_color.as_ptr();
        let _is_background: i32 = if is_background { 1 } else { 0 };
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Page_AddWatermark(
                self.pdfdocumentclass,
                num,
                c_char_ptr_text as *const c_char,
                c_char_ptr_font_name as *const c_char,
                font_size,
                c_char_ptr_foreground_color as *const c_char,
                x_position,
                y_position,
                rotation,
                _is_background,
                opacity,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::page_add_watermark({text:?}): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
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
    generate_fn!(_optimize_file_size, PDFDocument_OptimizeFileSize, image_quality: i32);
    generate_fn!(_repair, PDFDocument_Repair);
    generate_fn!(_grayscale, PDFDocument_Grayscale);
    generate_fn!(_flatten, PDFDocument_Flatten);
    generate_fn!(_embed_fonts, PDFDocument_EmbedFonts);
    generate_fn!(_unembed_fonts, PDFDocument_UnembedFonts);

    generate_fn!(_replace_text, PDFDocument_ReplaceText, find_text: &str, replace_text: &str);
    generate_fn!(_add_page_num, PDFDocument_AddPageNum);
    generate_fn!(_add_text_header, PDFDocument_AddTextHeader, header: &str);
    generate_fn!(_add_text_footer, PDFDocument_AddTextFooter, footer: &str);

    generate_fn!(_remove_annotations, PDFDocument_RemoveAnnotations);
    generate_fn!(_remove_attachments, PDFDocument_RemoveAttachments);
    generate_fn!(_remove_blank_pages, PDFDocument_RemoveBlankPages);
    generate_fn!(_remove_bookmarks, PDFDocument_RemoveBookmarks);
    generate_fn!(_remove_hidden_text, PDFDocument_RemoveHiddenText);
    generate_fn!(_remove_images, PDFDocument_RemoveImages);
    generate_fn!(_remove_javascripts, PDFDocument_RemoveJavaScripts);
    generate_fn!(_remove_tables, PDFDocument_RemoveTables);
    generate_fn!(_remove_watermarks, PDFDocument_RemoveWatermarks);

    generate_fn!(_decrypt, PDFDocument_Decrypt);

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
    generate_fn!(_page_remove_tables, PDFDocument_Page_RemoveTables, num: i32);
    generate_fn!(_page_remove_watermarks, PDFDocument_Page_RemoveWatermarks, num: i32);

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

    /// Optimizes size of PDF-document with image compression quality.
    ///
    /// # Arguments
    /// * `image_quality` - The image compression quality.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn optimize_file_size(&self, image_quality: i32) -> Result<(), PdfError> {
        self._optimize_file_size(image_quality)
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

    /// Flatten PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn flatten(&self) -> Result<(), PdfError> {
        self._flatten()
    }

    /// Embed fonts a PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn embed_fonts(&self) -> Result<(), PdfError> {
        self._embed_fonts()
    }

    /// Unembed fonts a PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn unembed_fonts(&self) -> Result<(), PdfError> {
        self._unembed_fonts()
    }

    /// Remove annotations from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_annotations(&self) -> Result<(), PdfError> {
        self._remove_annotations()
    }

    /// Remove attachments from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_attachments(&self) -> Result<(), PdfError> {
        self._remove_attachments()
    }

    /// Remove blank pages from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_blank_pages(&self) -> Result<(), PdfError> {
        self._remove_blank_pages()
    }

    /// Remove bookmarks from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_bookmarks(&self) -> Result<(), PdfError> {
        self._remove_bookmarks()
    }

    /// Remove hidden text from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_hidden_text(&self) -> Result<(), PdfError> {
        self._remove_hidden_text()
    }

    /// Remove images from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_images(&self) -> Result<(), PdfError> {
        self._remove_images()
    }

    /// Remove java scripts from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_javascripts(&self) -> Result<(), PdfError> {
        self._remove_javascripts()
    }

    /// Remove tables from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_tables(&self) -> Result<(), PdfError> {
        self._remove_tables()
    }

    /// Remove watermarks from PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn remove_watermarks(&self) -> Result<(), PdfError> {
        self._remove_watermarks()
    }

    /// Encrypt PDF-document.
    ///
    /// # Arguments
    /// * `user_password` - The user password.
    /// * `owner_password` - The owner password.
    /// * `permissions` - The allowed permissions (bitflags `Permissions`).
    /// * `crypto_algorithm` - The encryption algorithm (`CryptoAlgorithm` enum).
    /// * `use_pdf_20` - Whether to use PDF 2.0 encryption.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn encrypt(
        &self,
        user_password: &str,
        owner_password: &str,
        permissions: Permissions,
        crypto_algorithm: CryptoAlgorithm,
        use_pdf_20: bool,
    ) -> Result<(), PdfError> {
        debug_println!(
            "call Document::encrypt({permissions:?}, {crypto_algorithm:?}, {use_pdf_20:?})"
        );
        let c_string_user_password = std::ffi::CString::new(user_password).unwrap();
        let c_char_ptr_user_password = c_string_user_password.as_ptr();
        let c_string_owner_password = std::ffi::CString::new(owner_password).unwrap();
        let c_char_ptr_owner_password = c_string_owner_password.as_ptr();
        let _use_pdf_20: i32 = if use_pdf_20 { 1 } else { 0 };
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_Encrypt(
                self.pdfdocumentclass,
                c_char_ptr_user_password as *const c_char,
                c_char_ptr_owner_password as *const c_char,
                permissions.bits(),
                crypto_algorithm as i32,
                _use_pdf_20,
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::encrypt({permissions:?}, {crypto_algorithm:?}, {use_pdf_20:?}): {error_str:?})");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Decrypt PDF-document.
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn decrypt(&self) -> Result<(), PdfError> {
        self._decrypt()
    }

    /// Set permissions for PDF-document.
    ///
    /// # Arguments
    /// * `user_password` - The user password.
    /// * `owner_password` - The owner password.
    /// * `permissions` - The allowed permissions (bitflags `Permissions`).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn set_permissions(
        &self,
        user_password: &str,
        owner_password: &str,
        permissions: Permissions,
    ) -> Result<(), PdfError> {
        debug_println!("call Document::set_permissions({permissions:?})");
        let c_string_user_password = std::ffi::CString::new(user_password).unwrap();
        let c_char_ptr_user_password = c_string_user_password.as_ptr();
        let c_string_owner_password = std::ffi::CString::new(owner_password).unwrap();
        let c_char_ptr_owner_password = c_string_owner_password.as_ptr();
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        unsafe {
            PDFDocument_set_Permissions(
                self.pdfdocumentclass,
                c_char_ptr_user_password as *const c_char,
                c_char_ptr_owner_password as *const c_char,
                permissions.bits(),
                error.as_mut_ptr(),
            )
        };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(())
        } else {
            debug_println!("error Document::set_permissions({permissions:?})");
            Err(PdfError::CoreExceptionError(error_str))
        }
    }

    /// Get current permissions of PDF-document.
    ///
    /// # Returns
    /// * `Ok(Permissions)` - The bitmask of permissions.
    /// * `Err(PdfError)` - If the operation fails.
    pub fn get_permissions(&self) -> Result<Permissions, PdfError> {
        debug_println!("call Document::get_permissions()");
        let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
        let permissions_raw: i32 =
            unsafe { PDFDocument_get_Permissions(self.pdfdocumentclass, error.as_mut_ptr()) };
        let error_str = Self::get_error(&mut error);
        if error_str.is_empty() {
            Ok(Permissions::from(permissions_raw))
        } else {
            debug_println!("error Document::get_permissions(): {error_str:?}");
            Err(PdfError::CoreExceptionError(error_str))
        }
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

    /// Remove tables in page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_remove_tables(&self, num: i32) -> Result<(), PdfError> {
        self._page_remove_tables(num)
    }

    /// Remove watermarks in page.
    ///
    /// # Arguments
    /// * `num` - The page number (1-based).
    ///
    /// # Errors
    /// Returns `PdfError` if the operation fails.
    pub fn page_remove_watermarks(&self, num: i32) -> Result<(), PdfError> {
        self._page_remove_watermarks(num)
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
