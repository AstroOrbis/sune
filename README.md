# Sune

A proxy for Solana transactions or something idk

## Building

We use Just to make our lives easier and [Mold](https://github.com/rui314/mold) as a linker. After installing mold, update its path in .cargo/config.toml
using `just sync-mold-path`. Then just cargo b -r.
