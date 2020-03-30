#[macro_use]
extern crate blake3

// compute the hash of a string 
fn hash(val: str) -> str {
    let h = blake3::hash(val.as_bytes())
    hex::encode(h)
}
