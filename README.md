# contrast-checker

[![crates.io](https://img.shields.io/crates/v/contrast-checker.svg)](https://crates.io/crates/contrast-checker)

A simple Rust-based CLI tool for calculating the contrast ratio between two colours.

Colour contrast ratio is calculated based on [WCAG 2.0 guidelines](https://www.w3.org/TR/WCAG/#dfn-contrast-ratio).

# Installation

If you have [Rust](https://www.rust-lang.org/) installed, you can run
```
cargo install contrast-checker
```

# Usage

You can calculate the contrast between two colours by using:
```
contrast-checker <color1> <color2>
```

`color1` and `color2` are entered as either a Hex code without the `#` (i.e. `ffffff`) or as a comma separated RGB list (i.e. `255,255,255`).

For example:
```
contrast-checker ffffff 255,255,255
```