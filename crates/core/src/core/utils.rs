/*
    Appellation: utils <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::prelude::{rand::Rng, BoxResult};
use serde::{Deserialize, Serialize};
use std::io::Read;

/// From the given path, collect the file lines into a [Vec<String>]
pub fn extract_file_from_path(path: &str) -> Vec<String> {
    let mut file = match std::fs::File::open(std::path::Path::new(path.clone())) {
        Ok(f) => f,
        Err(e) => panic!("File Error: {}", e),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("File Error");
    buffer.split("\n").map(|s: &str| s.to_string()).collect()
}

/// Create a random set of elements from a source via index
pub fn generate_collection_from_reference(reference: Vec<String>, size: usize) -> Vec<String> {
    let mut cache = Vec::<String>::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        let random_index = rng.gen_range(0..reference.clone().len());
        cache.push(reference[random_index].clone())
    }
    cache
}

/// Quickly generate a random, secure string
pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

/// Save a serde enabled data-structure to a file
pub fn save_to_file<'a, T: Clone + Deserialize<'a> + Serialize>(
    data: T,
    path: &str,
) -> BoxResult<T> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    let buf_writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(buf_writer, &data)?;
    Ok(data.clone())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(4, 4);
        let expected: usize = 8;
        assert_eq!(actual, expected)
    }
}
