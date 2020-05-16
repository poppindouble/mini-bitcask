use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Result, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::command::Command;
use crate::constants::LOG_FILE_NAME;
use crate::utils::u8_array_to_u64;

pub struct Bitcask {
    pub reader: BitcaskReader,
    writer: BitcaskWriter,
    index: HashMap<Vec<u8>, Vec<u8>>,
}

struct BitcaskWriter {
    writer: BufWriter<File>,
}

impl BitcaskWriter {
    fn new(path: &PathBuf) -> Result<BitcaskWriter> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path.to_owned())?;
        let writer = BitcaskWriter {
            writer: BufWriter::new(file),
        };
        return Ok(writer);
    }

    fn fully_write(&mut self, buf: &mut Vec<u8>) -> Result<()> {
        let mut data_len = buf.len();

        while data_len > 0 {
            let written = self.writer.write(&buf)?;
            *buf = buf[written..].to_vec();
            data_len -= written;
        }

        return Ok(());
    }
}

impl Write for BitcaskWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl Seek for BitcaskWriter {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.writer.seek(pos)
    }
}

pub struct BitcaskReader {
    reader: BufReader<File>,
}

impl BitcaskReader {
    pub fn new(path: &PathBuf) -> Result<BitcaskReader> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&path.to_owned())?;
        let reader = BitcaskReader {
            reader: BufReader::new(file),
        };
        return Ok(reader);
    }
}

impl Read for BitcaskReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl Seek for BitcaskReader {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos)
    }
}

impl Bitcask {
    pub fn open(path: &PathBuf) -> Result<Bitcask> {
        create_dir_all(&path)?;

        let log_file_dir = get_log_file_dir(&path);

        let mut reader = BitcaskReader::new(&log_file_dir)?;
        let writer = BitcaskWriter::new(&log_file_dir)?;
        let mut index = HashMap::new();

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut current_pos = 0;

        while current_pos as i64 <= buffer.len() as i64 - 1 {
            let total_length_bytes = &buffer[current_pos..current_pos + 8];
            let total_length = u8_array_to_u64(&[
                total_length_bytes[0],
                total_length_bytes[1],
                total_length_bytes[2],
                total_length_bytes[3],
                total_length_bytes[4],
                total_length_bytes[5],
                total_length_bytes[6],
                total_length_bytes[7],
            ]);
            let bytes = &buffer[current_pos..current_pos + total_length as usize];

            let command = Command::from(bytes);
            match command {
                Command::Set { key, value } => {
                    index.insert(key, value);
                }
                Command::Remove { key } => {
                    index.remove(&key);
                }
            }

            current_pos += total_length as usize;
        }

        let bitcask = Bitcask {
            reader,
            writer,
            index,
        };

        return Ok(bitcask);
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let command = Command::Set { key, value };
        let command_bytes = command.parse();

        self.writer.fully_write(&mut command_bytes.to_vec())?;

        return Ok(());
    }

    pub fn get(&mut self, key: Vec<u8>) -> Option<Vec<u8>> {
        let res = self.index.get(&key)?;
        return Some(res.clone());
    }

    pub fn remove(&mut self, key: Vec<u8>) -> Result<()> {
        let command = Command::Remove { key };
        let command_bytes = command.parse();

        self.writer.fully_write(&mut command_bytes.to_vec())?;

        return Ok(());
    }
}

fn get_log_file_dir(dir: &PathBuf) -> PathBuf {
    let log_file_name = format!("{}.log", LOG_FILE_NAME);
    let log_path = dir.join(Path::new(&log_file_name));
    return log_path;
}
