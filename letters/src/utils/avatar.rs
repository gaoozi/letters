use base64::{engine::general_purpose, Engine};
use blake2::{Blake2s256, Digest};

pub fn _get_avatar_url(email: &str, size: i32) -> String {
    // create a Blake2b512 object
    let mut hasher = Blake2s256::new();
    // write input message
    hasher.update(email.as_bytes());
    // read hash digest and consume hasher
    let digest = general_purpose::STANDARD_NO_PAD.encode(hasher.finalize());
    format!(
        "https://avatars.dicebear.com/api/bottts/{}.svg?size={}",
        digest, size
    )
}
