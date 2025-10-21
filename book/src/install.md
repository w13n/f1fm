# installation
This page will guide you through installing F1FM on your machine.
Pre-compiled binaries are not currently built, so the build from source section is the only option for installation at the moment.

## Build from Source
The commands below assume a Mac, Linux, or WSL system.
These steps can be replicated on windows, but the commands will be different.

1. Ensure [Rust](https://rust-lang.org/) and Git are installed
2. Clone the [F1FM Repository](https://github.com/w13n/f1fm)

`git clone https://github.com/w13n/f1fm.git`

3. Navigate to the F1FM folder

`cd f1fm`

4. Compile and run the F1FM app

`cargo run --release`

5. **Optional**: Install F1FM

`cargo install --path .`
