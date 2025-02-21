use pbkdf2::hmac::digest::Digest;
use sha2::Sha256;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn hash_file(path: &Path) -> anyhow::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    let mut buf = [0; 4096];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }

        hasher.update(&buf[0..n]);
    }
    let hash = base16ct::lower::encode_string(&hasher.finalize());
    Ok(hash)
}
