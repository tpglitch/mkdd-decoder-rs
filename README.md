# mkdd_decoder

A Rust crate for decoding Mario Kart: Double Dash!! ghost data passwords

---

The game generates a 16-character password after a time trial that encodes the course kart, two drivers, total race time, and best lap time. This crate parses and validates those passwords.

Ported from the JavaScript decoder by [WaluigiBSOD](https://github.com/WaluigiBSOD/mkdd-password-decoder).

## Simple Usage

```rust
use mkdd_decoder::decode;

match decode("SOME16CHARPASSWD") {
    Ok(ghost) => println!("{}", ghost),
    Err(e)    => eprintln!("invalid password: {}", e),
}
```

## License

AGPL-3.0, see [LICENSE](LICENSE)
