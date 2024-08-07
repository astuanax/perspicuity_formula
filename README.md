# Perspicuity Formula

The Perspicuity Formula library provides a tool to calculate the readability score for Spanish texts based on the Fórmula de Perspicuidad. This formula helps determine the ease of understanding a given text, making it useful for writers, educators, and content creators who want to ensure their material is accessible to their intended audience.

```
use perspicuity_formula::PerspicuityFormula;

fn main() {
    let text = "Este es un texto de prueba. Está diseñado para verificar la fórmula de perspicuidad.";
    let pf = PerspicuityFormula::new();
    let score = pf.calculate(text);
    println!("Perspicuity Score: {}", score);
}
```

## Features

- Calculate the Perspicuity Score for Spanish texts.
- Automatically count sentences, words, and syllables in the text.
- Provides a score clamped between 0 and 100 for easy interpretation.
- Includes comprehensive tests for accuracy and reliability.

[![Crates.io](https://img.shields.io/crates/v/perspicuity_formula.svg)](https://crates.io/crates/perspicuity_formula)
[![Docs.rs](https://docs.rs/perspicuity_formula/badge.svg)](https://docs.rs/perspicuity_formula)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install perspicuity_formula`

## License

Licensed under:

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
