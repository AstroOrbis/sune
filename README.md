# Sune - INDEV

A proxy for Solana transactions or something

## Building

We use [Just](https://just.systems) to make our lives easier and [Mold](https://github.com/rui314/mold) as a linker. After installing mold, update its path in .cargo/config.toml
manually, or by using `just sync-mold-path`. Then just cargo b -r. The binary will then be in target/release/sune.

## Running

Args can be found by running `./sune --help`. The RPC defaults to the main public instance, and Jito defaults to the NYC instance.

For logs, run with `RUST_LOG=info`. For even more logs, run with `RUST_LOG=debug`.

Example:

```bash
RUST_LOG=info ./sune [your-args-here]
```
