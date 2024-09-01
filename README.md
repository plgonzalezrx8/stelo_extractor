# Apple Health Glucose Data Extractor

Glucose Data Extractor is a Rust-based tool designed to extract glucose data from Apple Health app exports. This tool allows users to easily isolate and analyze their glucose data using their preferred analysis tools.

## Features

- Extracts glucose data from Apple Health app exports
- Outputs data in JSON format for easy analysis
- Fast and efficient processing, even for large datasets
- Flexible command-line interface with options for input and output files

## Installation

1. Ensure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:

   ```bash
   git clone https://github.com/plgonzalezrx8/stelo_extractor.git
   ```

3. Navigate to the project directory:

   ```bash
   cd stelo_extractor
   ```

4. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

1. Export your Apple Health data from your iPhone:
   - Open the Health app
   - Tap your profile picture
   - Tap "Export All Health Data"
   - Choose a location to save the export

2. Extract the exported zip file and locate the `export.xml` file.

3. Run the Apple Health Glucose Data Extractor using one of the following methods:

   a. Specify input and output files:

   ```
   ./target/release/stelo_extractor --input /path/to/export.xml --output /path/to/output.json
   ```

   b. Use the current directory (assumes `export.xml` is in the current directory):

   ```
   ./target/release/stelo_extractor --current-directory
   ```

   This will look for `export.xml` in the current directory and save the output as `output.json` in the same directory.

4. Additional options:
   - Use `--help` to display the help menu with all available options:

     ```
     ./target/release/stelo_extractor --help
     ```

## Command-line Options

- `-i, --input <FILE>`: Specify the input XML file (required if not using --current-directory)
- `-o, --output <FILE>`: Specify the output JSON file (required if not using --current-directory)
- `--current-directory`: Use the current directory for input (`export.xml`) and output (`output.json`)
- `-h, --help`: Display help information
- `-V, --version`: Display version information

## Future Plans

We have several exciting features planned for future releases:

- Support for additional output formats (CSV, XLS, simplified XML)
- Graphical user interface (GUI) for easier interaction
- Cross-platform support (Windows, Linux, and macOS)
- Data visualization capabilities
- Integration with popular health and fitness apps

## Contributing

We welcome contributions to the Glucose Data Extractor project! If you're interested in contributing, please follow these steps:

1. Fork the repository
2. Create a new branch for your feature (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

If you have any questions or suggestions, please open an issue on this repository.

---

Thank you for using or contributing to Glucose Data Extractor!
