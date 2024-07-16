# A Rust collection that help access exchange rates

### Examples

**1. Get a rate**
```rust
fn is_there() {
    let e = ExchangeRates::from_rates(
        HashMap::from([
            (("USD", "EUR"), 1.)
        ])
    );
    assert_eq!(e.get("USD", "EUR"), Some(1.));
}
```

**2. Get a rate when you have the inverse**
```rust
fn reverse() {
    let e = ExchangeRates::from_rates(
        HashMap::from([
            (("USD", "EUR"), 2.)
        ])
    );
    assert_eq!(e.get("EUR", "USD"), Some(0.5));
}
```

**3. Get a A->C rate when you have A -> B and B -> C**
```rust
fn chain() {
    let e = ExchangeRates::from_rates(
        HashMap::from([
            (("USD", "JPY"), 2.),
            (("JPY", "EUR"), 3.),
        ])
    );
    assert_eq!(e.get("USD", "EUR"), Some(6.));
}
```
