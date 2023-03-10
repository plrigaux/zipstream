use rill::{
    archive::FileOptions, compress::tokio::archive::ZipArchive, compression::CompressionMethod,
    types::FileDateTime,
};
use std::io::Cursor;
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let file = File::create("archive.zip").await.unwrap();

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflate())
        .last_modified_time(FileDateTime::Now);
    let mut archive = ZipArchive::new(file);
    archive
        .append_file("file1.txt", &mut Cursor::new(b"hello\n"), &options)
        .await
        .unwrap();
    archive
        .append_file("file2.txt", &mut Cursor::new(b"world\n".to_vec()), &options)
        .await
        .unwrap();
    archive.finalize().await.unwrap();
}
