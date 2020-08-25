# Monitor CI

Web interfsace for monitoring the server running HCK-CI

# Installing Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Changing to nightly Rust toolchain (in order to use rocket.rs framework)

```
rustup toolchain install nightly
rustup default nightly
```

# Running in production

```
ROCKET_ENV=production cargo run
```
