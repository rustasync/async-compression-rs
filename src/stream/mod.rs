//! Types which operate over [`Stream`](futures::stream::Stream)`<Item =
//! `[`io::Result`](std::io::Result)`<`[`Bytes`](bytes::Bytes)`>>` streams, both encoders and
//! decoders for various formats.
//!
//! The `Stream` is treated as a single byte-stream to be compressed/decompressed, each item is a
//! chunk of data from this byte-stream. There is not guaranteed to be a one-to-one relationship
//! between chunks of data from the underlying stream and the resulting compressed/decompressed
//! stream, the encoders and decoders will buffer the incoming data and choose their own boundaries
//! at which to yield a new item.

mod flate;

pub mod brotli;
pub mod deflate;
pub mod gzip;
pub mod zlib;
