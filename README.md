# iter-scan

Iterator scan methods that don't suck.

## Motives

The `Iterator::scan` method that the Rust standard library provides are overcomplicated and inelegant. This crate aims to rectify this.

## Usage

The usage of this crate is included in the [documentation](https://docs.rs/iter-scan/latest/iter_scan/trait.IterScan.html).

## `no_std`

To use this crate with `no_std`, simply disable the `std` feature.

```toml
[dependencies.iter-scan]
version = "..."
default-features = false
features = []
```

## License

[MIT](https://github.com/KSXGitHub/iter-scan/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://ksxgithub.github.io/)
