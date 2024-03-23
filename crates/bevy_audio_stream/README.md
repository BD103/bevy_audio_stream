# `bevy_audio_stream`

[![crates.io](https://img.shields.io/crates/v/bevy_audio_stream)](https://crates.io/crates/bevy_audio_stream) [![docs.rs](https://docs.rs/bevy_audio_stream/badge.svg)](https://docs.rs/bevy_audio_stream)

This crate provides an alternative to the [Bevy] game engine's [`AudioSource`] type that enables asynchronously streaming audio programmatically. It is designed specifically for use in voice chat systems, but it can be applied elsewhere too.

[Bevy]: https://bevyengine.org/
[`AudioSource`]: https://docs.rs/bevy/latest/bevy/audio/struct.AudioSource.html

## Warning ⚠️

`bevy_audio_stream` is currently unstable and likely to contain breaking changes across minor releases until it stabilizes in version 1.0.0. Additionally, it was written by someone relatively inexperienced with audio. As such, it may not support your use-case or may contain bugs. If so, please [raise an issue] on Github and consider contributing! All help is appreciated. :)

[raise an issue]: https://github.com/BD103/bevy_mic

## Bevy Compatibility

|Bevy|`bevy_audio_stream`|
|-|-|
|0.13|0.1|

## License

`bevy_audio_stream` is dual-licensed under either

- [MIT License] (<http://opensource.org/licenses/MIT>)
- [Apache 2.0 License] (<http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

[MIT License]: https://github.com/BD103/bevy_mic/blob/main/LICENSE-MIT
[Apache 2.0 License]: https://github.com/BD103/bevy_mic/blob/main/LICENSE-APACHE

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
