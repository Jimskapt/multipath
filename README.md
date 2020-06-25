# multipath &emsp; [![crates_badge]][crates] [![doc_badge]][doc] [![MIT_badge]][LICENCE] [![Cargo tests]][GHA]

A library to split file path, like `/home/{user,admin}/file.txt`.

⚠ Warning : this version is not finished yet and should not be used in production. Take a look of [test accomplishments][GHA].

Example :

```rust
fn main() {
    assert_eq!(
        multipath::from("/home/{user,admin}/{Desktop,Download}/file.txt"),
        vec![
            "/home/user/Desktop/file.txt",
            "/home/user/Download/file.txt",
            "/home/admin/Desktop/file.txt",
            "/home/admin/Download/file.txt",
        ]
}
```

## Documentation

Please take a look to :

- [docs.rs API documentation][doc]
- [src/tests.rs][tests_GitHub]
  - Run tests with `cargo test`

[crates]: https://crates.io/crates/multipath
[crates_badge]: https://img.shields.io/crates/v/multipath
[doc]: https://docs.rs/multipath/
[doc_badge]: https://docs.rs/multipath/badge.svg
[LICENCE]: https://raw.githubusercontent.com/Jimskapt/multipath/master/LICENCE
[MIT_badge]: https://img.shields.io/badge/license-MIT-blue.svg
[Cargo tests]: https://github.com/Jimskapt/multipath/workflows/Cargo%20checks/badge.svg
[GHA]: https://github.com/Jimskapt/multipath/actions
[tests_GitHub]: https://github.com/Jimskapt/multipath/blob/master/examples/full_demo.rs
