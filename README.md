# Binkget: A wget Clone

Binkget is a command-line utility for downloading files over HTTPS, written in Rust. It is designed to be a minimal, efficient alternative to `wget`, leveraging the `tokio` asynchronous runtime and the `reqwest` HTTPS library for fast and concurrent downloads.

## Features
- Asynchronous, non-blocking downloads using `tokio`
- Progress bar visualization using `indicatif`
- Automatic filename specification
- Supports large file downloads with streaming
- Minimal dependencies for performance and reliability

## Installation

### Using Cargo
If you have Rust and Cargo installed, you can install Binkget directly from source:
```sh
cargo install binkget
```

### Building from Source
To build the binary manually, clone the repository and compile it using Cargo:
```sh
git clone https://github.com/yourusername/binkget.git
cd binkget
cargo build --release
```
The compiled binary will be available at `target/release/binkget`.

## Usage
Binkget allows you to download files from the command line by specifying a URL and a target filename.

### Basic Usage
```sh
binkget <URL> <FILENAME>
```
Example:
```sh
binkget https://ash-speed.hetzner.com/1GB.bin test.bin
```

### Command-line Options
- `-h` : Display help information.
- `-V` : Show the version number.

### Example Output
```
~/$ binkget https://ash-speed.hetzner.com/1GB.bin test.bin

URL: https://ash-speed.hetzner.com/1GB.bin
FILENAME: test.bin
HTTP request sent.. 200 OK
Length: 1073741824 (1.00 GiB)
Type: application/octet-stream
Saving to: test.bin
test.bin   [00:01:35] [========] 1.00 GiB/1.00 GiB eta: 0s
Download complete!
```

## How It Works

### Main Function
The main function sets up the CLI using `clap`, extracts command-line arguments, and initializes an asynchronous runtime to execute the `download` function.

### Download Process
1. Parses the URL.
2. Sends an HTTPS GET request using `reqwest`.
3. Reads the HTTP headers to determine the content length and type.
4. Displays the progress bar using `indicatif`.
5. Streams the response body asynchronously in chunks.
6. Writes the data incrementally to the output file.
7. Completes and finalizes the download.

## Dependencies
Binkget relies on the following Rust crates:
- `tokio`: Asynchronous runtime for handling non-blocking IO.
- `reqwest`: HTTP client for sending GET requests.
- `indicatif`: Progress bar visualization.
- `url`: URL parsing and validation.
- `console`: Terminal styling for improved readability.
- `futures-util`: Stream utilities for handling async data.

## Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

## License
Binkget is released under the MIT License.

## Contact
Author: Nathaniel Chappelle  
Email: [nathaniel.chappelle@proton.me](mailto:nathaniel.chappelle@proton.me)


