#[allow(unused_macros)]
#[macro_export]
macro_rules! generate_fn {
    // Case when there are no parameters
    ($fn_name:ident, $unsafe_fn:ident) => {
        fn $fn_name(&self) -> Result<(), PdfError> {
            debug_println!("call Document::{}", stringify!($fn_name));
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(self.pdfdocumentclass, error.as_mut_ptr());
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "error Document::{}(): {:?}",
                    stringify!($fn_name),
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with one `&str` parameter
    ($fn_name:ident, $unsafe_fn:ident, $param:ident: &str) => {
        fn $fn_name(&self, $param: &str) -> Result<(), PdfError> {
            debug_println!("Calling Document::{}({:?})", stringify!($fn_name), $param);
            let c_string = std::ffi::CString::new($param).unwrap();
            let c_char_ptr = c_string.as_ptr();
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(
                    self.pdfdocumentclass,
                    c_char_ptr as *const c_char,
                    error.as_mut_ptr(),
                );
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}): {:?}",
                    stringify!($fn_name),
                    $param,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with '&str', 'i32', 'i32' parameters
    ($fn_name:ident, $unsafe_fn:ident, $param:ident: &str, $param1:ident: i32, $param2:ident: i32) => {
        fn $fn_name(&self, $param: &str, $param1: i32, $param2: i32) -> Result<(), PdfError> {
            debug_println!(
                "Calling Document::{}({:?}, {:?}, {:?})",
                stringify!($fn_name),
                $param,
                $param1,
                $param2
            );
            let c_string = std::ffi::CString::new($param).unwrap();
            let c_char_ptr = c_string.as_ptr();
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(
                    self.pdfdocumentclass,
                    c_char_ptr as *const c_char,
                    $param1,
                    $param2,
                    error.as_mut_ptr(),
                );
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}, {:?}, {:?}): {:?}",
                    stringify!($fn_name),
                    $param,
                    $param1,
                    $param2,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with '&str', 'i32' parameters
    ($fn_name:ident, $unsafe_fn:ident, $param:ident: &str, $param1:ident: i32) => {
        fn $fn_name(&self, $param: &str, $param1: i32) -> Result<(), PdfError> {
            debug_println!(
                "Calling Document::{}({:?}, {:?})",
                stringify!($fn_name),
                $param,
                $param1
            );
            let c_string = std::ffi::CString::new($param).unwrap();
            let c_char_ptr = c_string.as_ptr();
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(
                    self.pdfdocumentclass,
                    c_char_ptr as *const c_char,
                    $param1,
                    error.as_mut_ptr(),
                );
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}, {:?}): {:?}",
                    stringify!($fn_name),
                    $param,
                    $param1,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with 'i32', 'i32' and '&str' parameters
    ($fn_name:ident, $unsafe_fn:ident, $param1:ident: i32, $param2:ident: i32, $param:ident: &str) => {
        fn $fn_name(&self, $param1: i32, $param2: i32, $param: &str) -> Result<(), PdfError> {
            debug_println!(
                "Calling Document::{}({:?}, {:?}, {:?})",
                stringify!($fn_name),
                $param1,
                $param2,
                $param
            );
            let c_string = std::ffi::CString::new($param).unwrap();
            let c_char_ptr = c_string.as_ptr();
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(
                    self.pdfdocumentclass,
                    $param1,
                    $param2,
                    c_char_ptr as *const c_char,
                    error.as_mut_ptr(),
                );
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}, {:?}, {:?}): {:?}",
                    stringify!($fn_name),
                    $param1,
                    $param2,
                    $param,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with `i32` and `&str` parameters
    ($fn_name:ident, $unsafe_fn:ident, $param1:ident: i32, $param:ident: &str) => {
        fn $fn_name(&self, $param1: i32, $param: &str) -> Result<(), PdfError> {
            debug_println!(
                "Calling Document::{}({:?}, {:?})",
                stringify!($fn_name),
                $param1,
                $param
            );
            let c_string = std::ffi::CString::new($param).unwrap();
            let c_char_ptr = c_string.as_ptr();
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(
                    self.pdfdocumentclass,
                    $param1,
                    c_char_ptr as *const c_char,
                    error.as_mut_ptr(),
                );
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}, {:?}): {:?}",
                    stringify!($fn_name),
                    $param1,
                    $param,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
    // Case with one `i32` parameter
    ($fn_name:ident, $unsafe_fn:ident, $param:ident: i32) => {
        fn $fn_name(&self, $param: i32) -> Result<(), PdfError> {
            debug_println!("Calling Document::{}({:?})", stringify!($fn_name), $param);
            let mut error: std::mem::MaybeUninit<*const c_char> = std::mem::MaybeUninit::uninit();
            unsafe {
                $unsafe_fn(self.pdfdocumentclass, $param, error.as_mut_ptr());
            }
            let error_str = Self::get_error(&mut error);
            if error_str.is_empty() {
                Ok(())
            } else {
                debug_println!(
                    "Error in Document::{}({:?}): {:?}",
                    stringify!($fn_name),
                    $param,
                    error_str
                );
                Err(PdfError::CoreExceptionError(error_str))
            }
        }
    };
}
