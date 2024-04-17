# Schoolhard

Faster and (hopefully) better mobile client for [Schoolsoft](https://www.schoolsoft.se/).

## Building
In order to build the project, you need to have the following installed:
- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/)
- [tauri-cli v2](https://crates.io/crates/tauri-cli/2.0.0-beta.12)

After you have installed the dependencies, you can build the project by running the following commands:
```bash
npm install
cargo tauri build
```

The built application will be located in the `src-tauri/target/release` directory.

## Development

In addition to the above dependencies, you will also need to have [typeshare-cli](https://crates.io/crates/typeshare-cli) installed, if you are going to be changing types in any way.

To start development, run the following command:
```bash
cargo tauri dev
```
