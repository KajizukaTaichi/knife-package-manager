# Knife

Knife is a simple, fast, and safe package manager written in Rust. It builds programs from source using `make`, ensuring that the software is tailored to your environment. Knife is designed to work in any environment and offers a straightforward installation process without requiring `sudo` privileges.

## Features

- **Custom Builds**: Every program is built from source using `make`, ensuring that it is optimized for your specific environment.
- **No Sudo Required**: Install packages easily with `knife install <program>`—no need for `sudo` privileges.
- **Easy Updates**: Keep Knife and your installed programs up to date with `knife update; knife upgrade`.
- **Fast and Safe**: Written in Rust, Knife provides a high-speed and secure package management experience.
- **Universal Compatibility**: Knife is designed to work in any environment.

## Installation

To install Knife, simply run the following command:

```bash
curl -sSf https://17do.github.io/knife-installer.github.io/sh.install.html | sh
```

## Usage

- **Install a package**: 
  ```bash
  knife install <program>
  ```

- **Update Knife**: 
  ```bash
  knife update; knife upgrade
  ```
  

## Contributing
Regarding contributions, the rules have not yet been determined.  
Please wait a little longer.

## License

This project is licensed under the MIT License—see the [LICENSE](LICENSE) file for details.
