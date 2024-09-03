use md5::{Digest, Md5};

pub fn md5_string(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}