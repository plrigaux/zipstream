use zipstream::{archive::Archive, compression::Compressor, types::FileDateTime};

mod common;
use common::create_new_clean_file;
const TEST_ID: &str = "NE";

async fn compress_file(compressor: Compressor, out_file_name: &str) {
    let file = create_new_clean_file(out_file_name).await;

    let mut archive = Archive::new(file);

    let mut in_file = tokio::fs::File::open("tests/file1.txt").await.unwrap();

    archive
        .append_file_no_extend("file1.txt", FileDateTime::Zero, compressor, &mut in_file)
        .await
        .unwrap();

    archive.finalize().await.unwrap();
    println!("archive size = {:?}", archive.get_archive_size());
    //let data = archive.finalize().await.unwrap();
}

#[tokio::test]
async fn archive_structure_compress_store() {
    let compressor = Compressor::Store();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_zlib_deflate_tokio() {
    let compressor = Compressor::Deflate();
    let out_file_name = ["test_", &compressor.to_string(), "_tokio", TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_zlib_deflate_flate2() {
    let compressor = Compressor::DeflateFate2();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, "_flate", ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_compress_bzip() {
    let compressor = Compressor::BZip2();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_compress_lzma() {
    let compressor = Compressor::Lzma();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_compress_zstd() {
    let compressor = Compressor::Zstd();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}

#[tokio::test]
async fn archive_structure_compress_xz() {
    let compressor = Compressor::Xz();
    let out_file_name = ["test_", &compressor.to_string(), TEST_ID, ".zip"].join("");

    compress_file(compressor, &out_file_name).await;
}