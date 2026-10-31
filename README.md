# SigmaOS Cinnamon Menus

This `no_std` crate currently exposes a `CinnamonMenus` value whose
constructor stores the fixed name `SigmaOS Cinnamon Menus`. It does not parse
desktop menu definitions, enumerate applications, or render a menu.

The inline unit test checks the stored name only; there is no menu integration
or graphical frontend in this crate.

## Build

```sh
cargo check --lib
```

## License

Apache-2.0. See [`Cargo.toml`](Cargo.toml).

## Inspiration

This component is inspired by Linux Mint and Omarchy Linux to provide:
- Single responsibility: One clear purpose
- Loose coupling: Minimal dependencies
- Stable interfaces: Well-defined APIs

## Status

- Architecture: Designed
- Implementation: Planned
- Testing: Planned
