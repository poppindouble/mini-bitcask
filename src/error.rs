use std::io::Error;

#[derive(Debug)]
pub enum KVError {
    IOError(Error),
    KeyNoneExisted,
}

impl From<Error> for KVError {
    fn from(err: Error) -> KVError {
        KVError::IOError(err)
    }
}

pub type KVResult<T> = Result<T, KVError>;
