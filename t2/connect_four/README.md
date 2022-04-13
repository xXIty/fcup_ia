# SYSTEM INFORMATION

- Language used: Rust
- Compiler version: `rustc 1.60.0`
- Pacage manager version: `cargo 1.60.0`
- Operating System used for development and testing:

```
Linux 5.16.12-arch1-1 #1 SMP PREEMPT Wed, 02 Mar 2022 12:22:51 +0000 x86_64 GNU/Linux
Linux 5.7.0-kali1-amd64 #1 SMP Debian 5.7.6-1kali2 (2020-07-01) x86_64 GNU/Linux
```

# INSTALLATION

If the machine does not have rust, install it with the distribution package manager or with this command:

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

To install the needed dependencies and build the program execute the following commands:

```bash
cargo build
```
The binary will be placed in `target/debug/connect_four`

# USAGE

To see usage run the following command:

``

# EXAMPLES

`connect_four -d 3 -a MCTS -s`

- `-d 3`: Make the algorithms run with a depth of 3
- `-a MCTS`: Choose the MCTS algorithm to play against.
- `-s`: Let the user play the first turn.
