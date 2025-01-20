# Rust  Backend

A project built to enhance my skills in **Rust** and **SurrealDB** during my layoff period. This project combines my interests in scalable database design and high-performance Rust development.


## Features
- **High Performance**: Leveraging Rust’s concurrency and memory safety for optimized performance.
- **Scalable Database**: Uses SurrealDB for handling structured, semi-structured, and graph-based data efficiently.
- **Flexible & Extensible**: Designed with modularity in mind, making it straightforward to extend or modify as needed.

## Motivation
After being laid off, I wanted to make the most of my time by deepening my knowledge in Rust and experimenting with SurrealDB. This project is a sandbox where I can explore scalable backend designs while improving my Rust programming skills.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/yourproject.git
    cd yourproject
    ```

2. Install dependencies and build:
    ```bash
    # Install Rust if not already installed
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup override set stable

    # Build the project
    cargo build --release
    ```

3. Set up SurrealDB:
    - Download and install SurrealDB from [https://surrealdb.com](https://surrealdb.com)
    - Start the SurrealDB instance and configure it as needed.

## Usage
Run the application with:
```bash
cargo run

## Generating RS256 Private and Public Keys

Follow these steps to generate and configure the necessary asymmetric keys:

---

#### 1. **Generate Keys**
1. Visit the [Online RSA Key Generator](https://travistidwell.com/jsencrypt/demo/).
2. Choose a **key size of 4096 bits** (recommended for compatibility with the `jsonwebtoken` crate).
   - Note: A 2048-bit key is acceptable but less secure. Avoid 1024-bit keys, as they may cause errors.
3. Click **"Generate New Keys"** and wait for the keys to be generated.

---

#### 2. **Configure Access Token Keys**
- **Private Key**:
  1. Copy the generated private key.
  2. Convert it to Base64 format using a tool like [Base64 Encode](https://www.base64encode.org/).
  3. Paste the Base64-encoded private key into your `.env` file as:
     ```
     ACCESS_TOKEN_PRIVATE_KEY=<Base64-encoded private key>
     ```

- **Public Key**:
  1. Copy the corresponding public key.
  2. Convert it to Base64 format.
  3. Paste the Base64-encoded public key into your `.env` file as:
     ```
     ACCESS_TOKEN_PUBLIC_KEY=<Base64-encoded public key>
     ```

---

#### 3. **Configure Refresh Token Keys**
Repeat the same steps for the refresh token keys:

- **Private Key**:
  1. Generate a new private key.
  2. Convert it to Base64 format.
  3. Add it to your `.env` file as:
     ```
     REFRESH_TOKEN_PRIVATE_KEY=<Base64-encoded private key>
     ```

- **Public Key**:
  1. Copy the corresponding public key.
  2. Convert it to Base64 format.
  3. Add it to your `.env` file as:
     ```
     REFRESH_TOKEN_PUBLIC_KEY=<Base64-encoded public key>
     ```

---

### Final Notes
- Ensure your `.env` file is secure and not exposed to unauthorized access.
- Use unique key pairs for access tokens and refresh tokens for better security.
- Test your application to confirm the keys are correctly integrated.