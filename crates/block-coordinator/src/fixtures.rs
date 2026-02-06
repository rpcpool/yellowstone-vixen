//! Fixture capture and replay for deterministic integration tests.
//!
//! Records raw `SubscribeUpdate` protobuf messages from the geyser stream
//! to a length-delimited file, then replays them in tests.
//!
//! File format: `[4 bytes: u32 BE message length][N bytes: SubscribeUpdate protobuf]...`

use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};

use prost::Message;
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, SubscribeUpdate};

/// Writes `SubscribeUpdate` messages to a length-delimited protobuf file.
///
/// Used at runtime during `--capture-fixtures` mode. Tracks the number of
/// `BlockMeta` messages seen to know when the target slot count is reached.
pub struct FixtureWriter {
    writer: BufWriter<File>,
    target_slots: usize,
    block_meta_count: usize,
}

impl FixtureWriter {
    pub fn new(path: &Path, target_slots: usize) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            target_slots,
            block_meta_count: 0,
        })
    }

    /// Encode and write a single `SubscribeUpdate` to the fixture file.
    ///
    /// Returns `true` if capture should continue, `false` when the target
    /// number of BlockMeta messages has been reached.
    pub fn write(&mut self, update: &SubscribeUpdate) -> io::Result<bool> {
        let bytes = update.encode_to_vec();
        let len = bytes.len() as u32;
        self.writer.write_all(&len.to_be_bytes())?;
        self.writer.write_all(&bytes)?;

        if matches!(
            update.update_oneof,
            Some(UpdateOneof::BlockMeta(_))
        ) {
            self.block_meta_count += 1;
            if self.block_meta_count >= self.target_slots {
                self.writer.flush()?;
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Reads `SubscribeUpdate` messages from a length-delimited protobuf file.
///
/// Implements `Iterator` for convenient replay in tests.
pub struct FixtureReader {
    reader: BufReader<File>,
}

impl FixtureReader {
    pub fn new(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }
}

impl Iterator for FixtureReader {
    type Item = SubscribeUpdate;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len_buf = [0u8; 4];
        match self.reader.read_exact(&mut len_buf) {
            Ok(()) => {},
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return None,
            Err(_) => return None,
        }

        let len = u32::from_be_bytes(len_buf) as usize;
        let mut msg_buf = vec![0u8; len];
        self.reader.read_exact(&mut msg_buf).ok()?;

        SubscribeUpdate::decode(&msg_buf[..]).ok()
    }
}
