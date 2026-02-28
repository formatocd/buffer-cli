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

1. Run the project. This can be done in two ways.
   - a. [Clone](#clone-repository) the project with git and [compile](#compilation) it, or [download](#download) the binary and run it.
   - b. [Clone](#clone-repository) the project and run the `cargo run` command in the root of the project.
2. When prompted, enter your Buffer API string/token.
3. Follow the on-screen instructions to select an organization and view its connected channels.

## Clone repository

**⚠️ Warning:** You must have [Git](https://git-scm.com/install/) installed on your system.

```bash
git clone https://github.com/formatocd/buffer_cli.git
```

## Compilation

**⚠️ Warning:** You must have [Rust](https://rust-lang.org/tools/install/) installed on your system.

To build an optimized release version of the application, run the following command:

```bash
cargo build --release
```

The compiled binary will be available in the `target/release/` directory.

## Download

You can also download the pre-compiled binaries from the [Releases](https://github.com/formatocd/buffer-cli/releases) section of this repository.