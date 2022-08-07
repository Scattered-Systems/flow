/*
    Appellation: constants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

/// Define the valid sizes of generated access grants
pub const ACCESS_GRANT_VALID_BIT_SIZES: [usize; 5] = [128, 160, 192, 224, 256];
/// Define the default filepath for locating the BIP0039 english text file
pub const PATH_TO_BIP0039_DATA: &str = "../../.artifacts/data/BIP0039/english.txt";
