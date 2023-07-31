# xplane-sys: Rust bindings for the X-Plane plugin SDK #

This library provides Rust bindings to the X-Plane plugin SDK.

## SDK versions ##

This crate is based on SDK version 4.0.1 (XPLM400), supporting X-Plane 12.04 and newer. Plugins made with this crate
can be used with earlier versions of X-Plane, so long as the right feature gates are set. The gates are as follows:

 * `XPLM400` -- X-Plane 12.04 and newer.
 * `XPLM303` -- X-Plane 11.50 and newer.
 * `XPLM301` -- X-Plane 11.20 and newer.
 * `XPLM300` -- X-Plane 11.10 and newer.
 * `XPLM210` -- X-Plane 10.20 and newer. (actually 10.00 and newer, but this crate forbids 32-bit plugins.)

Unlike the X-Plane C SDK, this crate automatically infers older versions from requesting a newer version, so no need
to define all the versions up to your desired version. Features from SDK versions newer than the requested version *can*
be used, if you perform a check on the SDK version level at runtime. At that point, if the version is high enough,
you can request a pointer to the desired function. This is outlined in the X-Plane documentation. This allows the
support of a range of versions, while also allowing access to newer features if present.

Some features from earlier SDK versions have been deprecated. This crate does not provide a way to access them.
Realistically, if you are using gates below `XPLM303`, you are nuts. The feature gates are included, but X-Plane 11.50
was released in 2020, so most users should have it, even if they are not on X-Plane 12.

This crate will be updated as the X-Plane SDK updates, and will have an equal version number.

## Documentation ##

The types and functions are documented on [the X-Plane plugin API website](http://developer.x-plane.com/sdk/).

## Compiling and linking ##

This crate currently can be compiled on macOS, Linux, and Windows. This is because X-Plane only (at the time of writing)
supports these platforms. This crate also only supports x86_64 and aarch64. These are the architectures supported by the
current version of X-Plane, and if you want to compile for ppc or iX86, you're on your own.

On macOS and Windows, plugins must be dynamically linked with libraries that
provide stub implementations of the SDK functions. This crate includes those
libraries and tells Cargo to link them.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Files in the SDK folder are provided under a separate license, provided in
[XPlaneSDK/license.txt](XPlaneSDK/license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
