extern crate blake3;

// compute the hash of a string 
fn hash(val: &str) -> String {
    let h = blake3::hash(val.as_bytes());
    hex::encode(h).to_string()
}

// generate a short code of a certain length from 
// a preset list of characters
fn gen_code(length: u16, alphabet: String) {
    
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() -> Result<(), String> {
        let (s, h) = ("aaa", "bbbb");
        assert_eq!(hash(s), h);
        Ok(())
    }
}
