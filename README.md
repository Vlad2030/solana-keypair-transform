# solana-keypair-transform

Simple tool for transform base58 private key from bytes list to string format and vice versa

# How to install

## Download rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
```

## Build and install

```bash
cargo test && cargo build --release
sudo cp target/release/solana-keypair-transform /bin/solana-keypair-transform

```

# Usage

```bash
solana-keypair-transform yVeoHry5k9Xe9SvjwXAnzuA4hSs5qwJ2WMRHqUhsk9MwcH6VDFLSN9eqAqNrUZ2YkNZNHe8qW8wf4FgzT3cC5Ys

[48,183,224,249,20,232,218,249,218,14,155,118,22,27,255,251,207,74,69,97,248,59,109,21,113,17,114,90,187,46,248,20,0,44,208,138,65,240,76,252,241,92,38,242,213,247,20,83,152,138,30,197,154,233,119,142,100,230,212,193,125,39,247,240]
```

and

```bash
solana-keypair-transform [48,183,224,249,20,232,218,249,218,14,155,118,22,27,255,251,207,74,69,97,248,59,109,21,113,17,114,90,187,46,248,20,0,44,208,138,65,240,76,252,241,92,38,242,213,247,20,83,152,138,30,197,154,233,119,142,100,230,212,193,125,39,247,240]

yVeoHry5k9Xe9SvjwXAnzuA4hSs5qwJ2WMRHqUhsk9MwcH6VDFLSN9eqAqNrUZ2YkNZNHe8qW8wf4FgzT3cC5Ys
```
