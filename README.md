# speedtest
`speedtest` is a command-line utility that measures a users download speed.

## Requirements
- **Rust**: The tool is written in Rust and it is required to compile.
- **Linux Environment**: Designed to work on Linux-based systems.

## Usage
```bash
speedtest [options]
```

### Options:
- `-h, --help`: Print help
- `-v, --version`: Print version

### Examples
1. Run speed test:
   ```bash
   speedtest
   ```

## Build and Install
To build the project, ensure you have `rust` installed, then compile the code as follows:

```bash
cd ./speedtest
```
```bash
cargo build --release
```
```bash
cargo install --path .
```

## Additional Information
- The download URL can be changed to anything

## License
This tool is licensed under the GNU General Public License (GPL). See ./LICENSE for more details.

## Contact
nbrandolino
nickbrandolino134@gmail.com
