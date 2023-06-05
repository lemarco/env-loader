# Simple storage for env variables with typings

- Depends on dotenv crate

Supported Values
```rust
pub enum Value {
    Str,    // String
    Int,    // i32
    Long,   // i64
    Bool,   // bool
}
```
Using
```rust
  let store = ConfigLoader::new(&[("LONG_VAR", Value::Long)]).unwrap();
  let num: i64 = store.get("LONG_VAR").unwrap();
```
