use std::{fs::File, path::Path};

use compstream::{
    archive::FileOptions, compress::std::archive::ZipArchiveNoStream,
    compression::CompressionMethod, error::ArchiveError,
};
mod common;
use common::std::create_new_clean_file;
const TEST_ID: &str = "NE";
const FILE_TO_COMPRESS: &str = "short_text_file.txt";

fn compress_file(compressor: CompressionMethod, out_file_name: &str) -> Result<(), ArchiveError> {
    let file = create_new_clean_file(out_file_name);

    let mut archive = ZipArchiveNoStream::new(file);

    let path = Path::new("tests/resources").join(FILE_TO_COMPRESS);

    let mut in_file = File::open(path)?;

    let options = FileOptions::default().compression_method(compressor);
    archive.append_file(FILE_TO_COMPRESS, &mut in_file, &options)?;

    archive.finalize()?;
    println!("archive size = {:?}", archive.get_archive_size());
    //let data = archive.finalize().unwrap();
    Ok(())
}

#[test]
fn archive_structure_compress_store() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::Store();
    let out_file_name = ["test_", TEST_ID, "_", &compressor.to_string(), ".zip"].join("");

    compress_file(compressor, &out_file_name)?;
    Ok(())
}

#[test]
fn archive_structure_zlib_deflate_tokio() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::Deflate();
    let out_file_name = [
        "test_",
        TEST_ID,
        "_",
        &compressor.to_string(),
        "_tokio",
        ".zip",
    ]
    .join("");

    compress_file(compressor, &out_file_name)?;
    Ok(())
}

#[test]
fn archive_structure_compress_bzip() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::BZip2();
    let out_file_name = ["test_", TEST_ID, "_", &compressor.to_string(), ".zip"].join("");

    compress_file(compressor, &out_file_name)?;
    Ok(())
}

#[test]
fn archive_structure_compress_lzma() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::Lzma();
    let out_file_name = ["test_", TEST_ID, "_", &compressor.to_string(), ".zip"].join("");

    match compress_file(compressor, &out_file_name) {
        Ok(_) => panic!("supposed not to be implemented"),
        Err(e) => {
            if matches!(e, ArchiveError::UnsuportedCompressionMethod(_)) {
                Ok(()) // This is of
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn archive_structure_compress_zstd() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::Zstd();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name)?;
    Ok(())
}

#[test]
fn archive_structure_compress_xz() -> Result<(), ArchiveError> {
    let compressor = CompressionMethod::Xz();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name)?;
    Ok(())
}