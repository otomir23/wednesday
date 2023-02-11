# Wednesday CLI

Wednesday is a simple CLI tool to scan Mojang's LauncherMeta API for a new Minecraft snapshot.
It is written in Rust and is my first Rust project, so I'm open to suggestions regarding implementation and optimisation.

## Download

You can find pre-built binaries [here](https://github.com/otomir23/wednesday/releases).

There's only Windows binaries right now, but I will try to build for other platforms if there will be a demand.

## Build it yourself

You need to install [Rust Toolchain](https://www.rust-lang.org/tools/install) to build the binaries.

 - Get the souce code
   
   ```bash
   git clone https://github.com/otomir23/wednesday.git
   ```
 -  Open the project
    
    ```bash
    cd wednesday
    ```
 - Build the binary
   
   ```bash
   cargo build --release
   ```
   
## License

This project is licensed under [MIT License](LICENSE).
