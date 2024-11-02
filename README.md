# derive(Description)

This library provides a trait and derive macro that is like [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html), but using compile-time strings.

The library is fully `no_std` and `no_alloc`, and is meant to provide user-facing text for enum-like status messages without code bloat.

```toml
[dependencies]
description = "0.3.0"
```

## Example

```rs
use description::Description;

#[derive(Description)]
enum ChargerStatus {
    #[description("Charger connected!")]
    Connected,

    #[description("Charger disconnected!")]
    Disconnected,
}

fn main() {
    let charger = ChargerStatus::Connected;

    println!("Charger notification: {}", charger.description());
}
```
[`std::fmt::format!()`](https://doc.rust-lang.org/std/macro.format.html)-like compile time formatting is also supported, thanks to [`const_format`](https://crates.io/crates/const_format/)

```rust
use description::Description;

const SOME_CONSTANT: usize = 5;

#[derive(Description)]
enum SomeStatusEnum {
    #[description("the constant is {SOME_CONSTANT}, and the max u32 is {}", u32::MAX)]
    ShowConstant,

    #[description("i'm not showing the constant")]
    DontShowConstant,
}

fn main() {
    let charger = SomeStatusEnum::ShowConstant;

    println!("enum message: {}", charger.description());
}
```