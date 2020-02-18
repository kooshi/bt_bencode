# BtBencode

BtBencode is a library which can help with [Bencode][wikipedia_bencode]
encoding/decoding.  Bencode is primarily used in [BitTorrent][bep_0003] related
applications.

It provides a [Serde][serde] serializer and deserializer.

## Installation

```toml
[dependencies]
bt_bencode = "0.1.0"
```

## License

Licensed under either of [Apache License, Version 2.0][LICENSE_APACHE] or [MIT
License][LICENSE_MIT] at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[LICENSE_APACHE]: LICENSE-APACHE
[LICENSE_MIT]: LICENSE-MIT
[wikipedia_bencode]: https://en.wikipedia.org/wiki/Bencode
[bep_0003]: http://www.bittorrent.org/beps/bep_0003.html
[serde]: https://serde.rs
