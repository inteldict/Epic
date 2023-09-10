# EPIC: A CGI Implementation of EPOS Parser

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)

# Introduction

For the CGI module a browser sends parameters in form of ```QUERY_STRING``` variable.
Here is an example of expected string:
```
QUERY_STRING=s=Sch%C3%B6n,%20Sie%20kennenzulernen&num=10
```
The query string consist of two variables ```s``` (string) and ```num``` (int).

# Build

```
cargo build --release
```

# Usage

```agsl
QUERY_STRING="s=Sch%C3%B6n%20Sie%20kennenzulernen&num=1" ./epic.cgi
```

[//]: # (```toml)

[//]: # ([dependencies])

[//]: # (config = "0.13.1")

[//]: # (```)

[//]: # ()
[//]: # (### Feature flags)

[//]: # ()
[//]: # ( - `ini` - Adds support for reading INI files)

[//]: # ( - `json` - Adds support for reading JSON files)

[//]: # ( - `yaml` - Adds support for reading YAML files)

[//]: # ( - `toml` - Adds support for reading TOML files)

[//]: # ( - `ron` - Adds support for reading RON files)

[//]: # ( - `json5` - Adds support for reading JSON5 files)

[//]: # ()
[//]: # (### Support for custom formats)

[//]: # ()
[//]: # (Library provides out of the box support for most renowned data formats such as JSON or Yaml. Nonetheless, it contains an extensibility point - a `Format` trait that, once implemented, allows seamless integration with library's APIs using custom, less popular or proprietary data formats.)

[//]: # ()
[//]: # (See [custom_format]&#40;https://github.com/mehcode/config-rs/tree/master/examples/custom_format&#41; example for more information.)

[//]: # ()
[//]: # (### More)

[//]: # ()
[//]: # (See the [documentation]&#40;https://docs.rs/config&#41; or [examples]&#40;https://github.com/mehcode/config-rs/tree/master/examples&#41; for)

[//]: # (more usage information.)

## MSRV

We currently support Rust 1.64.0 and newer.

## License

EPIC is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
