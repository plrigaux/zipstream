use crate::async_write_wrapper::AsyncWriteWrapper;
use async_compression::tokio::write::BzEncoder;
use async_compression::tokio::write::LzmaEncoder;
use async_compression::tokio::write::XzEncoder;
use async_compression::tokio::write::ZlibEncoder;
use async_compression::tokio::write::ZstdEncoder;
use crc32fast::Hasher;
use flate2::write::ZlibEncoder as ZlibEncoderFlate;
use flate2::Compression;
use std::fmt::Display;
use std::io::Error as IoError;
use std::io::Write;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub const STORE: u16 = 0;
pub const DEFALTE: u16 = 8;
pub const BZIP2: u16 = 12;
pub const LZMA: u16 = 14;
pub const ZSTD: u16 = 93;
pub const XZ: u16 = 95;

#[derive(Debug)]
pub enum Compressor {
    Store(),
    Deflate(),
    DeflateFate2(),
    BZip2(),
    Lzma(),
    Zstd(),
    Xz(),
    Unknown(u16),
}

impl Compressor {
    pub fn compression_method(&self) -> u16 {
        match self {
            Compressor::Store() => STORE,
            Compressor::Deflate() => DEFALTE,
            Compressor::DeflateFate2() => DEFALTE,
            Compressor::BZip2() => BZIP2,
            Compressor::Lzma() => LZMA,
            Compressor::Zstd() => ZSTD,
            Compressor::Xz() => XZ,
            Compressor::Unknown(compression_method) => *compression_method,
        }
    }

    pub fn version_needed(&self) -> u16 {
        // higher versions matched first
        match self {
            Compressor::BZip2() => 46,
            _ => 20,
        }
    }

    pub fn from_compression_method(compression_method: u16) -> Compressor {
        // higher versions matched first
        match compression_method {
            STORE => Compressor::Store(),
            DEFALTE => Compressor::Deflate(),
            BZIP2 => Compressor::BZip2(),
            LZMA => Compressor::Lzma(),
            ZSTD => Compressor::Zstd(),
            XZ => Compressor::Xz(),
            _ => Compressor::Unknown(compression_method),
        }
    }

    pub fn compression_method_label(&self) -> &str {
        // higher versions matched first
        match self {
            Compressor::Store() => "store",
            Compressor::Deflate() => "deflate",
            Compressor::DeflateFate2() => "deflate",
            Compressor::BZip2() => "bzip2",
            Compressor::Lzma() => "lzma",
            Compressor::Zstd() => "zstd",
            Compressor::Xz() => "xz",
            Compressor::Unknown(_) => "unknown",
        }
    }

    pub async fn compress<'a, R, W>(
        &self,
        writer: &'a mut AsyncWriteWrapper<W>,
        reader: &'a mut R,
        hasher: &'a mut Hasher,
    ) -> Result<usize, IoError>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        match self {
            Compressor::Store() => {
                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    writer.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                //w.flush().await?;
                //w.shutdown().await?;

                Ok(total_read)
            }
            Compressor::Deflate() => {
                let mut zencoder = ZlibEncoder::new(writer);

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                zencoder.flush().await?;
                zencoder.shutdown().await?;

                Ok(total_read)
            }

            Compressor::DeflateFate2() => {
                //TODO chage vec to stream
                let mut zencoder = ZlibEncoderFlate::new(Vec::new(), Compression::default());

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read])?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }

                zencoder.flush()?;

                let hello = zencoder.finish()?;

                writer.write_all(&hello).await?;
                writer.flush().await?;

                Ok(total_read)
            }

            Compressor::BZip2() => {
                let mut zencoder = BzEncoder::new(writer);

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                zencoder.shutdown().await?;

                Ok(total_read)
            }
            Compressor::Lzma() => {
                let mut zencoder = LzmaEncoder::new(writer);

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                zencoder.shutdown().await?;

                Ok(total_read)
            }
            Compressor::Zstd() => {
                let mut zencoder = ZstdEncoder::new(writer);

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                zencoder.shutdown().await?;

                Ok(total_read)
            }
            Compressor::Xz() => {
                let mut zencoder = XzEncoder::new(writer);

                let mut buf = vec![0; 4096];
                let mut total_read = 0;

                loop {
                    let read = reader.read(&mut buf).await?;
                    if read == 0 {
                        break;
                    }

                    total_read += read;
                    hasher.update(&buf[..read]);
                    zencoder.write_all(&buf[..read]).await?;
                    //self.sink.write_all(&buf[..read]).await?; // Payload chunk.
                }
                zencoder.shutdown().await?;

                Ok(total_read)
            }
            Compressor::Unknown(compression_method) => {
                panic!("unsupported compression method {:?}", compression_method)
            }
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Compressor::Unknown(_))
    }
}

impl Display for Compressor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.compression_method_label())
    }
}
