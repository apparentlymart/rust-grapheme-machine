# GraphemeMachine: Grapheme Cluster Segmentation state machine in Rust

This is a Rust library implementaing of the Grapheme Cluster portion of
[UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/),
which prioritizes streaming-friendliness and simplicity.

This library implements the segmentation algorithm as of Unicode 16.0.0,
using the character database tables from that release.

For more information, refer to
[the API documentation](https://docs.rs/grapheme_machine/latest/grapheme_machine/).
