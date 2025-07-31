use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufReader, Read},
    os,
    path::Path,
};

use bzip2::read::BzDecoder;
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let lib_dir = if let Ok(custom) = env::var("ASPOSE_PDF_LIB_DIR") {
        Path::new(&custom).to_path_buf()
    } else {
        Path::new(&manifest_dir).join("lib")
    };

    // Ensure lib directory exists
    if !lib_dir.exists() {
        return Err(format!("Library directory not found: {}", lib_dir.display()).into());
    }

    let target = env::var("TARGET")?;

    // Platform-specific library file name
    let lib_name = match target.as_str() {
        "x86_64-pc-windows-msvc" => "AsposePDFforRust_windows_amd64.dll",
        "x86_64-unknown-linux-gnu" => "libAsposePDFforRust_linux_amd64.so",
        "aarch64-apple-darwin" => "libAsposePDFforRust_darwin_arm64.dylib",
        "x86_64-apple-darwin" => "libAsposePDFforRust_darwin_amd64.dylib",
        _ => return Err(format!("Unsupported target platform: {}", target).into()),
    };

    let lib_path = lib_dir.join(&lib_name);

    // If library does not exist, attempt to extract from .bz2 archive
    if !lib_path.exists() {
        let compressed_path = lib_dir.join(format!("{}.bz2", lib_name));
        let checksum_path = lib_dir.join(format!("{}.bz2.sha256", lib_name));

        if !compressed_path.exists() {
            return Err(format!(
                "Missing both library and compressed archive: {}",
                compressed_path.display()
            )
            .into());
        }

        if !checksum_path.exists() {
            return Err(format!(
                "Missing checksum file for archive: {}",
                checksum_path.display()
            )
            .into());
        }

        println!(
            "Verifying SHA256 for archive: {}",
            compressed_path.display()
        );

        // Read expected SHA256 hash from .sha256 file
        let sha256_contents = fs::read_to_string(&checksum_path)?;
        let expected_sha256 = sha256_contents
            .trim()
            .split_whitespace()
            .next()
            .ok_or("Invalid SHA256 file format")?;

        // Compute actual SHA256 hash of the archive
        let mut archive_file = File::open(&compressed_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let count = archive_file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        let actual_sha256 = hex::encode(hasher.finalize());
        if actual_sha256 != expected_sha256 {
            return Err(format!(
                "SHA256 mismatch for {}:\nExpected: {}\nActual:   {}",
                compressed_path.display(),
                expected_sha256,
                actual_sha256
            )
            .into());
        }

        // Decompress the archive to extract the library
        println!("Decompressing {}", compressed_path.display());
        let archive_file = File::open(&compressed_path)?;
        let mut decoder = BzDecoder::new(BufReader::new(archive_file));
        let mut out_file = File::create(&lib_path)?;
        std::io::copy(&mut decoder, &mut out_file)?;
        println!("Decompressed to {}", lib_path.display());
    }

    // Final existence check
    if !lib_path.exists() {
        return Err(format!(
            "Library still missing after decompression: {}",
            lib_path.display()
        )
        .into());
    }

    // Prepare library name for linking (strip prefix and suffix)
    let lib_base_name = lib_name
        .trim_start_matches("lib")
        .trim_end_matches(".so")
        .trim_end_matches(".dylib")
        .trim_end_matches(".dll");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={}", lib_base_name);
    println!("cargo:rerun-if-changed={}", lib_dir.display());

    // Platform-specific rpath handling
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    // Symlink library to build output directory (so executable can find it)
    let out_dir = env::var("OUT_DIR")?;
    let exe_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .ok_or("Cannot determine exe dir")?;
    let dst_lib = exe_dir.join(&lib_name);

    if fs::symlink_metadata(&dst_lib).is_err() {
        #[cfg(unix)]
        let _ = os::unix::fs::symlink(&lib_path, &dst_lib);
        #[cfg(windows)]
        let _ = os::windows::fs::symlink_file(&lib_path, &dst_lib);
    }

    Ok(())
}
