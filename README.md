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

## MSRV

We currently support Rust 1.65.0 and newer.

## License

EPIC is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
