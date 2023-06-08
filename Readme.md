# Simple storage for env variables with typings

- Depends on dotenv crate

ConfigLoader::new(..) returns error if one or more values cannot be read from .env file or any of provided constraints are violated; 
It is expected behaviour because you don't want to start application without required env values.

If you're trying to get a value with a type that mismatched type in schema you will get an error.
Package should not panic in any case.

## Using with macro
```rust
  let env_values = convert! {
            PORT: int,           // typing is anything possible to lovercase to i32, int, integer: Int,int,INT,Integer,I32,etc..
            HOST: str,           // same rule for str | string       
            CRITICAL_FLAG: bool, // same rule for bool | boolean
            LONG_VAR: i64        // same rule for i64 | long
        };
  let store = ConfigLoader::new(env_values, None).unwrap(); // second arg for custom env file
  let port: i32 = store.get("PORT").unwrap();
  let host: String = store.get("HOST").unwrap();
  let flag: bool = store.get("CRITICAL_FLAG").unwrap();
  let num: i64 = store.get("LONG_VAR").unwrap();
```

### Supported types
- [int , integer, i32, Int, Integer ...]  as i32 in rust
- [long, Long, i64...]                    as i64 in rust
- [str, string, String, Str...]           as String in rust
- [bool, boolean]                         as bool in rust
## Using with constraints
After name of variable and type devided by ':' you can add constraints devided by "=>" 
```rust
  let env_values = convert! {
            PORT: int => min(1000) max(2000),      
            HOST: str => min(10),             
            CRITICAL_FLAG: bool => optional, 
            LONG_VAR: i64  =>  min(10000),  
            NOT_EMPTY_STR_VALUE:str => notEmpty
        };

```
### Supported constraints:
- min() for string, int, long
- max() for string, int, long
- notEmpty for string
- optional for all types

Trailing comma is not supported.
## Using with custom file
```rust
  let env_values = convert! {
            PORT: int,
            HOST: str,  
        };
  let store = ConfigLoader::new(env_values, Some(".env.test")).unwrap();
  let port: i32 = store.get("PORT").unwrap();
  let host: String = store.get("HOST").unwrap();
```

- If you find env-loader useful in your projects, I kindly request your support by starring the corresponding Git repository. 
- Additionally, I welcome you to actively engage with the repository by opening issues if you encounter any bugs, inconsistencies, or areas for improvement. Your input is immensely valuable, as it helps me identify and resolve any issues promptly. Furthermore, if you have proposals or ideas for enhancing the functionality of env-loader, please don't hesitate to share them as well. I believe in collaborative development and welcome your contributions to make this crate even more exceptional.

