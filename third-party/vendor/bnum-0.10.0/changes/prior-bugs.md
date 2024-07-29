- `<= v0.1.0`: incorrect implementation of `from_be`, `to_be` on all integers.
- `<= v0.8.0`: incorrect implementation of `unchecked_shl`, `unchecked_shr` for `BUint` with `64` bit digits.
- `<= v0.8.0`: incorrect implementation of `Add<$Digit>` for all `BUint`s (where `$Digit` is the underlying digit type).
- `<= v0.8.0`: `lcm` incorrectly panics when `self` is zero.
- `v0.7.0 - v0.9.1`: the implementations of `shr, shl, wrapping_{shr, shl}, overflowing_{shr, shl}, checked_{shr, shl}, rotate_left, rotate_right, from_be_slice, from_le_slice` methods and any methods that involved left-shift or right-shift operations, contained undefined behaviour (this was due to a premature stabilisation of a feature in the Rust compiler - see <https://github.com/rust-lang/rust/pull/117905> and <https://github.com/isaacholt100/bnum/issues/36>). Note that all tests for these methods were still passing for each relevant version, so it is very likely that the code was still working correctly. However, as of the time of writing, the code that they relied on will soon be rejected by the compiler. They have now been replaced with safe implementations which are just as fast.