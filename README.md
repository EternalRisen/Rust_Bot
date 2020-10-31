# `Rust-Bot`

## `TODOs`
 - add an actual bot to here.(done)
 - implement trivia commands and functions
#

## `Getting Started`
- Dependencies
    - make sure you have `cargo` installed by installing it respectively with your OS/distro
    - anything else should be satisfied by `Cargo.toml`
- Token and other important variables
    - You can do this in either of two ways:
        - `export DISCORD_TOKEN=your_token_here` from the command line
        - or by structuring a `.env` file correctly in the root directory
            ```
            DISCORD_TOKEN=your_token_here
            ```
- Running the bot:
    - You can do this either of two ways:
        - Running `cargo run` in the root directory (use the `--release` if you running this for production)
        - Or doing `cargo build` and executing the binary in `target/[build, release]`  depending on if you provide the `--release` flag
