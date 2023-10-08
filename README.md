# chronostore

[![CI](https://github.com/endoli/chronostore.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/endoli/chronostore.rs/actions/workflows/ci.yml)
[![](https://img.shields.io/crates/v/chronostore.svg)](https://crates.io/crates/chronostore)
[![docs.rs](https://img.shields.io/docsrs/chronostore)](https://docs.rs/chronostore)

Chronostore is a system for storing time series in memory.

What is Chronostore NOT?

* It does not try to be a distributed system.
* It does not have failover.
* It doesn't run as a separate process out of the box.
* It doesn't even persist data to disk automatically.

So, what *is* Chronostore good for?

When you need a smaller scale storage of data that is
timestamped, Chronostore is useful.

Once data has been collected from a primary source
such as profiling samplers or counters, program tracing,
hardware counters, or other sources of high frequency,
high precision data, it is often useful to have it in
a form that tools can work with for analyzing and
visualizing that data.

The initial implementation is quite naive and is just
here to get something working. Over time, the implementation
will evolve and become significantly more sophisticated.

Chronostore must be fast at inserts, fast at queries,
and memory efficient.

Dual licensed under the MIT and Apache 2 licenses.

## Documentation

The API is fully documented with examples: <https://docs.rs/chronostore/>

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/chronostore).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
chronostore = "0.0.1"
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
