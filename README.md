# derive(Description)

This library provides a trait and derive macro that is like [``], but using compile-time strings.

The library is fully `no_std` and `no_alloc`, and is meant to provide user-facing text for enum-like status messages without code bloat.

```toml
[dependencies]
description = "0.1.0"
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