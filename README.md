# contrast-checker

A simple Rust-based CLI tool for calculating the contrast ratio between two colours.

Colour contrast ratio is calculated based on [WCAG 2.0 guidelines](https://www.w3.org/TR/WCAG/#dfn-contrast-ratio).

# Usage

You can calculate the contrast between two colours by using:
`contrast-checker <color1> <color2>`

`color1` and `color2` are entered as either a Hex code without the `#` (ex. `ffffff`) or as a comma separated RGB list (ex. `255,255,255`).

For example:
`contrast-checker ffffff 255,255,255`