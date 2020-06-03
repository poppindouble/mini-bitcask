use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir, File, OpenOptions, DirEntry};
use std::io::{BufReader, BufWriter, Read, Result, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::error::{KVError, KVResult};
use crate::storage::bitcask::command::Command;
use crate::storage::bitcask::log_pointer::LogPointer;
use crate::utils::u8_array_to_u64;

pub struct Bitcask {
    readers: HashMap<u64, BitcaskReader>,
    writer: BitcaskWriter,
    index: HashMap<Vec<u8>, LogPointer>,
    current_gen: u64,
    uncompacted: u64,
}

struct BitcaskWriter {
    writer: BufWriter<File>,
}

impl BitcaskWriter {
    fn new(path: &PathBuf) -> KVResult<BitcaskWriter> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path.to_owned())?;
        let writer = BitcaskWriter {
            writer: BufWriter::new(file),
        };
        return Ok(writer);
    }

    fn fully_write(&mut self, buf: &mut Vec<u8>) -> KVResult<()> {
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

struct BitcaskReader {
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
    pub fn open(path: &PathBuf) -> KVResult<Bitcask> {
        create_dir_all(&path)?;

        let sorted_gen_list = get_sorted_gen_list(&path)?;

        let mut index = HashMap::new();
        let mut readers = HashMap::new();
        let mut uncompacted = 0;

        for gen in &sorted_gen_list {
            uncompacted += load_index(*gen, &path, &mut readers, &mut index)?;
        }

        let current_gen = {
            if sorted_gen_list.is_empty() {
                0
            } else {
                *sorted_gen_list.last().unwrap()
            }
        };

        let writer_log_path = get_log_file_dir(current_gen, &path);
        let writer = BitcaskWriter::new(&writer_log_path)?;

        let bitcask = Bitcask {
            readers,
            writer,
            index,
            current_gen,
            uncompacted,
        };

        return Ok(bitcask);
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> KVResult<()> {
        let command = Command::Set {
            key: key.clone(),
            value,
        };

        let command_bytes = command.parse();

        let current_pos = self.writer.seek(SeekFrom::Current(0))?;
        let command_bytes_len = command_bytes.len();

        let log_pointer = LogPointer::new(self.current_gen, current_pos, command_bytes_len as u64);

        self.index.insert(key, log_pointer);

        self.writer.fully_write(&mut command_bytes.to_vec())?;
        self.writer.flush()?;

        return Ok(());
    }

    pub fn get(&mut self, key: Vec<u8>) -> KVResult<Option<Vec<u8>>> {
        match self.index.get(&key) {
            None => Err(KVError::KeyNoneExisted),
            Some(log_pointer) => {
                match self.readers.get_mut(&log_pointer.gen) {
                    Some(reader) => {
                        reader.seek(SeekFrom::Start(log_pointer.pos))?;

                        let mut log_pointer_reader = reader.by_ref().take(log_pointer.len);

                        let mut buffer = vec![0; log_pointer.len as usize];
                        log_pointer_reader.read(&mut buffer)?;

                        let command = Command::from(buffer.as_slice());

                        match command {
                            Command::Set { key: _, value } => {
                                return Ok(Some(value));
                            }
                            Command::Remove { key: _ } => {
                                return Ok(None);
                            }
                        }
                    }
                    None => {
                        return Ok(None);
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: Vec<u8>) -> KVResult<()> {
        let command = Command::Remove { key: key.clone() };
        let command_bytes = command.parse();

        match self.index.remove(&key) {
            Some(old_log_pointer) => {
                self.uncompacted += old_log_pointer.len;
                self.uncompacted += *&command_bytes.len() as u64;
            }
            None => {}
        }

        self.writer.fully_write(&mut command_bytes.to_vec())?;

        return Ok(());
    }
}

fn get_sorted_gen_list(path: &PathBuf) -> KVResult<Vec<u64>> {
    let x = read_dir(&path)?;

    let res: Vec<u64> = x.map(|yo| {
//        return Ok(yo?.path());
        return yo?.path().to_string_lossy().parse::<i32>();
    }).filter_map(|ff| Result::ok)
        .map(|j| {
        return 3;
    }).collect();


    let s = vec![0];
    return Ok(s);

//    let mut entries: Vec<u64> = read_dir(&path)?
//        .map(|maybe_dir_entry| Ok(maybe_dir_entry?))
//        .filter(|dir_entry| {})
//        .map(|entry| {
//            let file_name = entry.file_name().into_string()?;
//            let gen = file_name.parse::<u64>()?;
//            return gen;
//        })
//        .collect();
//
//    entries.sort();
//
//    return Ok(entries);
}

fn load_index(
    gen: u64,
    path: &PathBuf,
    readers: &mut HashMap<u64, BitcaskReader>,
    index: &mut HashMap<Vec<u8>, LogPointer>,
) -> KVResult<u64> {
    let mut uncompacted = 0;
    let log_path = get_log_file_dir(gen, &path);
    let mut reader = BitcaskReader::new(&log_path)?;

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
        ]) as usize;
        let bytes = &buffer[current_pos..current_pos + total_length];

        let command = Command::from(bytes);
        match command {
            Command::Set { key, value: _ } => {
                let log_pointer = LogPointer::new(gen, current_pos as u64, total_length as u64);
                if let Some(old_log_pointer) = index.insert(key, log_pointer) {
                    uncompacted += old_log_pointer.len;
                }
            }
            Command::Remove { key } => {
                if let Some(old_log_pointer) = index.remove(&key) {
                    uncompacted += old_log_pointer.len;
                }
                uncompacted += total_length as u64;
            }
        }

        current_pos += total_length as usize;
    }

    readers.insert(gen, reader);

    return Ok(uncompacted);
}

fn get_log_file_dir(gen: u64, dir: &PathBuf) -> PathBuf {
    let log_file_name = format!("{}.log", gen);
    let log_path = dir.join(Path::new(&log_file_name));
    return log_path;
}
