# Question

[![Crates.io](https://img.shields.io/crates/v/question.svg)](https://crates.io/crates/question) [![Crates.io](https://img.shields.io/crates/d/question.svg)](https://crates.io/crates/question) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/sevro/question/blob/master/LICENSE) [![Coverage Status](https://codecov.io/gl/starshell/question/branch/master/graph/badge.svg)](https://codecov.io/gl/starshell/question)

Linux: [![Build status](https://gitlab.com/starshell/question/badges/master/pipeline.svg)](https://github.com/sevro/question/commits/master)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/k7ccce79080tfu18/branch/master?svg=true)](https://ci.appveyor.com/project/Eudoxier/question/branch/master)

A Command Line Question Asker for Rust.

> Ask a question, what more could you want?

Easy to use library for asking users questions when writing console/terminal applications.

**This package is passively maintained.** Don't let the inactivity fool you issues and feature requests will be addressed. It is due to being a small focused library that just works so well nothing has had to be done.

## Usage

Add `question` as a dependency in your `Cargo.toml` to use from crates.io:

```toml
[dependencies]
question = "0.2.2"
```

Then add `extern crate question;` to your crate root and run `cargo build` or `cargo update && cargo build` for your project. Detailed documentation for releases can be found on [docs.rs](https://docs.rs/question/). 

### Example

```rust
extern crate question;
use question::{Answer, Question};

fn main() {
    let answer = Question::new("Continue?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
        println!("Onward then!");
    } else {
        println!("Aborting...");
    }
}
```

Examples can also be ran directly:

```sh
$ cargo run --example yes_no_with_defaults
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/yes_no_with_defaults`
Continue? (Y/n) why
Continue? (Y/n) y
Onward then!
```

See [examples](examples/) for more.

## Contributing

To contribute to Question, please see [CONTRIBUTING](CONTRIBUTING.md).

## License

Question is distributed under the terms of the MIT license. If this does not suit your needs for some reason please feel free to contact me, or open a [discussion](https://github.com/sevro/question/discussions/categories/general).

See [LICENSE](LICENSE).
