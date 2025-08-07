# GhostPass

> Your ephemeral, encrypted, zero-trace vault in the terminal.  
> Built with Rust. Trusted by ghosts.

**GhostPass** is a modern, minimalist command-line password manager built in Rust. It offers cutting-edge zero-residency security principles, ensuring your sensitive secrets never linger in memory or disk unencrypted. Designed for privacy, usability, and resilience, GhostPass stands apart from traditional password managers by guaranteeing that *nothing is ever left behind*.

---

## Features

- **Zero-Residency Security**  
  Secrets never reside in plaintext memory or disk unencrypted. Secrets are handled securely using Rust's `secrecy` crate and encrypted with AES-GCM before any persistence.

- **Fast & Lightweight CLI Interface**  
  Easy to use commands: `add`, `get`, `delete`, `list`. Built with Rust's `clap` crate for a modern CLI experience.

- **Encrypted, Versioned Vault**  
  Vault data is AES-GCM encrypted and safely persisted in a single file (`ghostpass_vault.dat`). Designed for future extension with version control and audit trails.

- **Clipboard Shadowing & Auto-Clear**  
  When retrieving a password, it automatically copies to the clipboard and clears it after 10 seconds to reduce exposure.

- **Cryptographic Trust Fingerprint (Planned)**  
  Visual cryptographic cues will help prevent phishing attacks, letting you verify vault integrity easily.

- **Extensible & Open Architecture**  
  Easily integrate biometric unlock, USB hardware tokens (FIDO2), and cloud syncing in the future.

---

## Installation

Ensure you have Rust and Cargo installed. Then:

# Clone the repo
```bash
git clone https://github.com/yourusername/ghostpass.git
cd ghostpass

# Build the release binary
cargo build --release

# Check CLI options
./target/release/ghostpass --help

```
# Usage
On each run, you will be prompted to enter your master password, which derives your encryption key.

Add a new entry
```bash
./target/release/ghostpass add <key> <password>
```

Example:

```bash
./target/release/ghostpass add github mySuperSecretPassword
```
Retrieve a password
```bash
./target/release/ghostpass get <key>
```
Example:

```bash
./target/release/ghostpass get github
```
This copies the password to your clipboard for 10 seconds and then clears it automatically to minimize risk.

Delete an entry
```bash
./target/release/ghostpass delete <key>
```
List all saved keys
```bash
./target/release/ghostpass list
```
##Security Notes
-All secrets are encrypted using AES-GCM with keys derived from your master password.

-Secrets are kept in memory only temporarily, protected by the secrecy crate and zeroed out after use.

-Clipboard contents are auto-cleared after a timeout to prevent lingering sensitive data.

-Secrets are only shown explicitly when requested (such as with get), balancing usability with safety.

-Planned upgrades include hardware token unlocks and encrypted cloud syncing.

##Contributing
Contributions, bug reports, and feature requests are welcome. Please open an issue or submit a pull request.



##Contact
Questions or consulting? Reach out at yatharthsingh1444@gmail.com.
