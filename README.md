# rusty-join
Exercise for Efficient Programs 2024

## Data
For local development run:
```mkdir data && cp /localtmp/efficient24 ~/rusty-join/data"```
This puts all the necessary files into the data folder.

## SSH development
Go to `~/.ssh` and add your local PC's public key to the file `authorized_keys`.
With `ssh-keygen` you can also create a keypair for g0 machine and add the public key to Github under:
- Settings/SSH and GPG keys

Afterwards you should not need your password everytime.
Additionally for apple users `ssh-add --apple-use-keychain ~/.ssh/{yourkey}` on the local machine is needed I think.

Also it may be required to install rustup for lsp support in your editor on the remote:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
Installs rustc and cargo and rust-analyzer
## Functionality tests
- `cargo test` ... sanity test for the current impl

## LVA Benchmarking
- `LC_NUMERIC=en_US perf stat -e cycles /usr/ftp/pub/anton/lvas/effizienz-aufgabe24/myjoin /localtmp/efficient24/f1.csv /localtmp/efficient24/f2.csv /localtmp/efficient24/f3.csv /localtmp/efficient24/f4.csv > /dev/null`

## Caro Benchmarking (may be overkill)
- `cargo build --release` ... build release
- `cargo bench` ... run benchmarks
  -  to add more benchmark files go to benches/mybench.rs
  - add
  ```toml
  [[bench]]
  name = "simple"
  harness = false
  ```

  to `Cargo.toml`
- `cargo bench -- --verbose` ... run benchmarks with more output
- `cargo install cargo-benchcmp` .. install benchcmp
  - `cargo bench > control`
  - `cargo bench > variable`
  - `cargo benchcmp`
  - `scp -r ep12124528@g0.complang.tuwien.ac.at:/home/ep24/ep12124528/rusty-join/target/criterion/ ./ && cd criterion && http-server`
  - `cargo bench | awk '!/^[a-z],[a-z],[a-z],[a-z],[a-z]$/' > output.txt` run cargo bench and output to output.txt because stdout is unreadable
