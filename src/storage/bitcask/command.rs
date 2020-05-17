use crate::utils::{u64_to_u8_array, u8_array_to_u64};
use std::mem::size_of;

enum CommandPrefix {
    Set = 0x00,
    Remove = 0x01,
}

pub enum Command {
    Set { key: Vec<u8>, value: Vec<u8> },
    Remove { key: Vec<u8> },
}

impl Command {
    pub fn parse(&self) -> Vec<u8> {
        match self {
            Command::Set { key, value } => {
                let mut res = Vec::new();
                let command_type_byte = CommandPrefix::Set as u8;
                let command_key_size = key.len() as u64;
                let command_value_size = value.len() as u64;
                let total_size = size_of::<u64>()
                    + size_of::<u8>()
                    + size_of::<u64>()
                    + key.len()
                    + size_of::<u64>()
                    + value.len();

                res.append(&mut u64_to_u8_array(total_size as u64).to_vec());
                res.push(command_type_byte);
                res.append(&mut u64_to_u8_array(command_key_size).to_vec());
                res.append(&mut key.clone());
                res.append(&mut u64_to_u8_array(command_value_size).to_vec());
                res.append(&mut value.clone());

                return res;
            }
            Command::Remove { key } => {
                let mut res = Vec::new();
                let command_type_byte = CommandPrefix::Remove as u8;
                let command_key_size = key.len() as u64;
                let total_size = size_of::<u64>() + size_of::<u8>() + size_of::<u64>() + key.len();

                res.append(&mut u64_to_u8_array(total_size as u64).to_vec());
                res.push(command_type_byte);
                res.append(&mut u64_to_u8_array(command_key_size).to_vec());
                res.append(&mut key.clone());
                return res;
            }
        }
    }
}

impl From<&[u8]> for Command {
    fn from(data: &[u8]) -> Command {
        let mut _current_pos = 0;

        let _total_size = &data[_current_pos.._current_pos + size_of::<u64>()];

        _current_pos += size_of::<u64>();

        let command_type_byte = &data[_current_pos.._current_pos + size_of::<u8>()];
        _current_pos += size_of::<u8>();

        let command_key_size_bytes = &data[_current_pos.._current_pos + size_of::<u64>()];
        let command_key_size = u8_array_to_u64(&[
            command_key_size_bytes[0],
            command_key_size_bytes[1],
            command_key_size_bytes[2],
            command_key_size_bytes[3],
            command_key_size_bytes[4],
            command_key_size_bytes[5],
            command_key_size_bytes[6],
            command_key_size_bytes[7],
        ]) as usize;
        _current_pos += size_of::<u64>();

        let key_bytes = &data[_current_pos.._current_pos + command_key_size];
        _current_pos += command_key_size;

        if command_type_byte[0] == CommandPrefix::Set as u8 {
            let command_value_size_bytes = &data[_current_pos.._current_pos + size_of::<u64>()];
            let command_value_size = u8_array_to_u64(&[
                command_value_size_bytes[0],
                command_value_size_bytes[1],
                command_value_size_bytes[2],
                command_value_size_bytes[3],
                command_value_size_bytes[4],
                command_value_size_bytes[5],
                command_value_size_bytes[6],
                command_value_size_bytes[7],
            ]) as usize;
            _current_pos += size_of::<u64>();

            let value_bytes = &data[_current_pos.._current_pos + command_value_size];
            _current_pos += command_value_size;

            Command::Set {
                key: key_bytes.to_vec(),
                value: value_bytes.to_vec(),
            }
        } else {
            Command::Remove {
                key: key_bytes.to_vec(),
            }
        }
    }
}
