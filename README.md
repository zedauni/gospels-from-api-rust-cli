# Tiny cli to get Gospel data from api

Command-line application written in Rust that extracts gospel data from an API based on a specific date, an entire month, or a range of months. The extracted data is saved as cleaned local JSON files.

## Features

- Extract gospel data for a specific date.
- Extract gospel data for an entire month.
- Extract gospel data for a range of months.
- Save results in JSON format.

## Usage

### Download latest release



Then run the program using the following options:

### Get gospel data for a specific date
```bash
./gospels --day YYYY-MM-DD

Ex: ./gospels 2024-12-25
```

### Get gospels data for an entire month
```bash
./gospels --month YYYY-MM

Ex: ./gospels 2024-12
```

### Get gospels data for a range of months
```bash
./gospels --range YYYY-MM YYYY-MM

Ex: ./gospels 2024-12 2025-02
```

JSON files will be generated in the current directory.

### Current Help output (gospels -h)

```
gospels 0.1.0
Gospels via API

USAGE:
    gospels [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <day>           Date spécifique à traiter (YYYY-MM-DD)
    -m, --month <month>       Mois à traiter (YYYY-MM)
    -r, --range <range>...    Plage de mois à traiter (YYYY-MM YYYY-MM)
```

## Example of generated content

For a specific date, a JSON file will be created with the following structure:

```json
{
  "2024-03-10": [
    {
      "content": "...Car Dieu a tellement aimé le monde<br />\nqu’il a donné son Fils unique,<br />\nafin que quiconque croit en lui ne se perde pas,<br />\nmais obtienne la vie éternelle...",
      "titre": "« Dieu a envoyé son Fils pour que, par lui, le monde soit sauvé »",
      "ref": "Jn 3, 14-21"
    }
  ]
}
```

## Development

1. Prerequisites for Development
  - [Rust](https://www.rust-lang.org/tools/install) installed on your system.
  - Internet connection to access the API.

2. Clone the repository:
   ```bash
   git clone https://github.com/zedauni/gospels-from-api-rust-cli.git
   cd gospels-from-api-rust-cli
   ```

3. Run the project:
   ```bash
   cargo run
   ```

4. Or build the project:
   ```bash
   cargo build --release
   ```

## Contributions

Contributions are welcome! Feel free to open an issue or a pull request to improve the project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

In the current version, the API used is that provided by [AELF](https://api.aelf.org). Please note that [AELF grants Web users the right to reproduce all or part of the content of the site for storage for the purposes of representation on a single-user screen and reproduction, in one copy, for backup copies or printing on paper](https://www.aelf.org/page/conditions-generales-dutilisation). This right is restricted to strictly personal, private and non-collective use, and any networking, redistribution or total or partial marketing to third parties is strictly forbidden in any form whatsoever.

It is your responsibility to comply with the General Conditions of Use of the AELF website and data. Otherwise, you must modify the project source code to use the API of your choice. 

Thanks to [API AELF](https://www.aelf.org/page/mentions-legales) for providing liturgical data.