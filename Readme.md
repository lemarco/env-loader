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
Using with enum
```rust
  let store = ConfigLoader::new(&[("LONG_VAR", Value::Long)]).unwrap();
  let num: i64 = store.get("LONG_VAR").unwrap();
```

Using with macro
```rust
  let env_values = convert_values! {
            PORT: int,           // typing is anything possible to lovercase to i32, str, string: Int,int,INT,Integer,I32,etc..
            HOST: str,           // same rule for str | string       
            CRITICAL_FLAG: bool, // same rule for bool | boolean
            LONG_VAR: i64        // same rule for i64 | long
        };
  let store = ConfigLoader::new(&env_values).unwrap();
  let port: i32 = store.get("PORT").unwrap();
  let host: String = store.get("HOST").unwrap();
  let flag: bool = store.get("CRITICAL_FLAG").unwrap();
  let num: i64 = store.get("LONG_VAR").unwrap();
```

- If you find env-loader useful in your projects, I kindly request your support by starring the corresponding Git repository. 
- Additionally, I welcome you to actively engage with the repository by opening issues if you encounter any bugs, inconsistencies, or areas for improvement. Your input is immensely valuable, as it helps me identify and resolve any issues promptly. Furthermore, if you have proposals or ideas for enhancing the functionality of env-loader, please don't hesitate to share them as well. I believe in collaborative development and welcome your contributions to make this crate even more exceptional.

