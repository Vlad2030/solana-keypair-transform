use clap::Parser;
use thiserror::Error;

#[derive(Parser, Debug)]
#[clap(
    name = "solana-keypair-transform",
    version = "0.2.0",
    author = "https://github.com/Vlad2030/solana-keypair-transform/",
    about = "Tool for transform base58 private key from bytes list to string format and vice versa"
)]
struct Cli {
    keypair: String,
}

static ARRAY_KEYPAIR_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^\[.*\]$").unwrap());
static STRING_KEYPAIR_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{43,88}$").unwrap());

#[derive(Clone, Debug, Error)]
pub enum KeypairError {
    #[error("Invalid keypair format")]
    InvalidFormat,
    #[error("Failed to parse byte: {0}")]
    ParsingFailed(String),
    #[error("Keypair transformation failed: {0}")]
    TransformationFailed(String),
}

#[derive(Clone, Debug)]
pub enum Base58Keypair {
    Array(Vec<u8>),
    String(String),
}

impl Base58Keypair {
    pub fn transform(self) -> Result<String, KeypairError> {
        match self {
            Self::Array(s) => ed25519_dalek::Keypair::from_bytes(&s)
                .map(|keypair| bs58::encode(keypair.to_bytes()).into_string())
                .map_err(|_| {
                    KeypairError::TransformationFailed("Invalid keypair array".to_string())
                }),
            Self::String(s) => bs58::decode(&s)
                .into_vec()
                .map_err(|_| {
                    KeypairError::TransformationFailed(
                        "Failed to decode base58 string".to_string(),
                    )
                })
                .and_then(|decoded| {
                    ed25519_dalek::Keypair::from_bytes(&decoded)
                        .map(|keypair| {
                            keypair
                                .to_bytes()
                                .iter()
                                .map(|byte| byte.to_string())
                                .collect::<Vec<String>>()
                                .join(",")
                        })
                        .map_err(|_| {
                            KeypairError::TransformationFailed(
                                "Invalid keypair string".to_string(),
                            )
                        })
                }),
        }
    }

    pub fn from_string(keypair: String) -> Result<Self, KeypairError> {
        if ARRAY_KEYPAIR_REGEX.is_match(&keypair) {
            let stripped = keypair
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .ok_or(KeypairError::InvalidFormat)?;

            let vec = stripped
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<u8>()
                        .map_err(|_| KeypairError::ParsingFailed(s.to_string()))
                })
                .collect::<Result<Vec<u8>, _>>()?;

            return Ok(Self::Array(vec));
        }

        if STRING_KEYPAIR_REGEX.is_match(&keypair) {
            return Ok(Self::String(keypair));
        }

        Err(KeypairError::InvalidFormat)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("");

    let keypair = Base58Keypair::from_string(cli.keypair).map_err(|e| e)?;

    let transformed_keypair = keypair.clone().transform().map_err(|e| e)?;

    match keypair {
        Base58Keypair::String(_) => println!("[{}]", transformed_keypair),
        Base58Keypair::Array(_) => println!("{}", transformed_keypair),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_from_string_array() {
        let input = "[48,183,224,249,20,232,218,249,218,14,155,118,22,27,255,251,207,74,69,97,248,59,109,21,113,17,114,90,187,46,248,20,0,44,208,138,65,240,76,252,241,92,38,242,213,247,20,83,152,138,30,197,154,233,119,142,100,230,212,193,125,39,247,240]";
        let keypair = Base58Keypair::from_string(input.to_string());

        assert!(keypair.is_ok());
    }

    #[test]
    fn keypair_from_string() {
        let input = "yVeoHry5k9Xe9SvjwXAnzuA4hSs5qwJ2WMRHqUhsk9MwcH6VDFLSN9eqAqNrUZ2YkNZNHe8qW8wf4FgzT3cC5Ys";
        let keypair = Base58Keypair::from_string(input.to_string());

        assert!(keypair.is_ok());
    }

    #[test]
    fn transform_keypair_from_string_array() {
        let input = "[48,183,224,249,20,232,218,249,218,14,155,118,22,27,255,251,207,74,69,97,248,59,109,21,113,17,114,90,187,46,248,20,0,44,208,138,65,240,76,252,241,92,38,242,213,247,20,83,152,138,30,197,154,233,119,142,100,230,212,193,125,39,247,240]";
        let keypair = Base58Keypair::from_string(input.to_string());
        let transformed = keypair.unwrap().transform();

        assert!(transformed.is_ok());
    }

    #[test]
    fn transform_keypair_from_string() {
        let input = "yVeoHry5k9Xe9SvjwXAnzuA4hSs5qwJ2WMRHqUhsk9MwcH6VDFLSN9eqAqNrUZ2YkNZNHe8qW8wf4FgzT3cC5Ys";
        let keypair = Base58Keypair::from_string(input.to_string());
        let transformed = keypair.unwrap().transform();

        assert!(transformed.is_ok());
    }

    #[test]
    fn valid_transform_keypair_from_string_array() {
        let input = "[48,183,224,249,20,232,218,249,218,14,155,118,22,27,255,251,207,74,69,97,248,59,109,21,113,17,114,90,187,46,248,20,0,44,208,138,65,240,76,252,241,92,38,242,213,247,20,83,152,138,30,197,154,233,119,142,100,230,212,193,125,39,247,240]";
        let keypair = Base58Keypair::from_string(input.to_string());
        let transformed = keypair.unwrap().transform();
        let is_valid = STRING_KEYPAIR_REGEX.is_match(transformed.unwrap().as_str());

        assert!(is_valid);
    }

    #[test]
    fn valid_transform_keypair_from_string() {
        let input = "yVeoHry5k9Xe9SvjwXAnzuA4hSs5qwJ2WMRHqUhsk9MwcH6VDFLSN9eqAqNrUZ2YkNZNHe8qW8wf4FgzT3cC5Ys";
        let keypair = Base58Keypair::from_string(input.to_string());
        let transformed = keypair.unwrap().transform();
        let is_valid =
            ARRAY_KEYPAIR_REGEX.is_match(format!("[{}]", transformed.unwrap()).as_str());

        assert!(is_valid);
    }
}
