# Buffer CLI

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/tokio-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![GraphQL](https://img.shields.io/badge/-GraphQL-E10098?style=for-the-badge&logo=graphql&logoColor=white)

A simple command-line interface tool written in Rust to interact with the [Buffer GraphQL API](https://developers.buffer.com/guides/getting-started.html).

## Features

- **Authentication**: Prompts the user securely for their Buffer API Key.
- **Organization Selection**: Retrieves all organizations associated with the user's account. If multiple organizations are found, it allows the user to interactively select which one to query.
- **Channel Listing**: Fetches and displays all social media channels (e.g., Twitter, LinkedIn, Facebook) linked to the selected organization, providing their service names and IDs.

## Technologies Used

- **[Rust](https://www.rust-lang.org/)**: Core programming language.
- **[Tokio](https://tokio.rs/)**: Asynchronous runtime for Rust.
- **[Reqwest](https://docs.rs/reqwest/latest/reqwest/)**: Ergonomic, batteries-included HTTP Client.
- **[Serde JSON](https://docs.rs/serde_json/latest/serde_json/)**: Strongly typed JSON framework.

## Usage

1. Clone the repository and navigate to the project directory.
```bash
git clone https://github.com/formatocd/buffer_cli.git
cd buffer_cli
```
2. Run the project locally using Cargo:
```bash
cargo run
```

3. When prompted, enter your Buffer API string/token.
4. Follow the on-screen instructions to select an organization and view its connected channels.

## Compilation

To build an optimized release version of the application, use the following commands based on your operating system:

**Windows & macOS:**
```bash
cargo build --release
```

**Linux:**
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

The compiled binary will be available in the `target/release/` or `target/x86_64-unknown-linux-musl/release/` directory.

## Download

You can also download the pre-compiled binaries from the [Releases](https://github.com/formatocd/buffer-cli/releases) section of this repository.