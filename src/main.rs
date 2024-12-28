use clap::Parser;
use solana_sdk::signer::Signer;

#[derive(Parser, Debug)]
#[clap(
    name = "solana-keypair-transform",
    version = "0.1.0",
    author = "https://github.com/Vlad2030/solana-keypair-transform/",
    about = "Simple tool for transform secret to string format and vice versa"
)]
struct Cli {
    keypair: String,
}

#[derive(Clone)]
enum Base58KeypairType {
    Array(Vec<u8>),
    String(String),
}

impl Base58KeypairType {
    fn transform(self) -> Result<solana_sdk::signer::keypair::Keypair, String> {
        match self {
            Self::Array(s) => solana_sdk::signer::keypair::Keypair::from_bytes(&s)
                .map_err(|_| "Invalid byte array for Keypair".to_string()),
            Self::String(s) => Ok(solana_sdk::signer::keypair::Keypair::from_base58_string(&s)),
        }
    }
}

struct Base58Keypair;

impl Base58Keypair {
    fn array_regex() -> regex::Regex {
        regex::Regex::new(r"^\[.*\]$").unwrap()
    }

    fn string_regex() -> regex::Regex {
        regex::Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{43,88}$").unwrap()
    }

    fn from_string(keypair: String) -> Result<Base58KeypairType, ()> {
        if Self::array_regex().is_match(keypair.as_str()) {
            let stripped = keypair
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .unwrap();
            let vec = stripped
                .split(',')
                .map(|s| s.trim())
                .map(|s| {
                    s.parse::<u8>()
                        .map_err(|_| format!("Failed to parse byte: {}", s))
                })
                .collect::<Result<Vec<u8>, _>>();

            if vec.is_ok() {
                return Ok(Base58KeypairType::Array(vec.unwrap()));
            }
        }

        if Self::string_regex().is_match(keypair.as_str()) {
            return Ok(Base58KeypairType::String(keypair));
        }

        Err(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let keypair = Base58Keypair::from_string(cli.keypair);

    if keypair.is_err() {
        println!("Error parsing");
        return Ok(());
    }

    let transformed_keypair = keypair.clone().unwrap().transform();

    if transformed_keypair.is_err() {
        println!(
            "Error transforming: {}",
            transformed_keypair.err().unwrap().as_str()
        );
        return Ok(());
    }

    match keypair.unwrap() {
        Base58KeypairType::String(_) => {
            let mut keypair: Vec<String> = transformed_keypair
                .as_ref()
                .unwrap()
                .secret()
                .to_bytes()
                .iter()
                .map(|byte| byte.to_string())
                .collect::<Vec<String>>();
            keypair.extend(
                transformed_keypair
                    .as_ref()
                    .unwrap()
                    .pubkey()
                    .to_bytes()
                    .iter()
                    .map(|byte| byte.to_string())
                    .collect::<Vec<String>>(),
            );
            println!("[{}]", keypair.join(","));
            Ok(())
        }
        Base58KeypairType::Array(_) => {
            println!("{}", transformed_keypair.unwrap().to_base58_string());
            Ok(())
        }
    }
}
