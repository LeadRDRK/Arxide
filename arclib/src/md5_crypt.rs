use std::io::{Read, Write};

use md5::{Digest, Md5};

use crate::Error;

const KEY_LEN: usize = 43;

pub struct MD5Crypt {
    pub key: [u8; KEY_LEN]
}

impl MD5Crypt {
    pub fn new(k: &[u8]) -> Result<MD5Crypt, Error> {
        if k.len() != KEY_LEN {
            return Err(Error::InvalidKeyLength(KEY_LEN, k.len()));
        }

        let mut key = [0u8; KEY_LEN];
        key.copy_from_slice(k);

        Ok(MD5Crypt {
            key
        })
    }

    pub fn apply(&self, path_md5: &str, input: &mut impl Read, output: &mut impl Write) -> Result<(), Error> {
        // yup, we md5 the md5 string
        let mut hasher = Md5::new();
        hasher.update(path_md5);
        let mut key_index = (hasher.finalize()[7] as usize) % KEY_LEN;

        let mut buf = [0u8; 1];
        loop {
            let read_size = input.read(&mut buf)?;
            if read_size == 0 {
                return Ok(());
            }

            buf[0] ^= self.key[key_index];
            key_index = (key_index + 1) % KEY_LEN;

            output.write(&buf)?;
        }
    }
}