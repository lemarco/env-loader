#[cfg(test)]

mod tests {
    use crate::*;
    #[test]
    fn check_test_file() {
        let store = ConfigLoader::new(
            convert! {
                PORT:i32=>min(1000) max(10000)
            },
            Some(".env.test"),
        )
        .unwrap();
        let port: i32 = store.get("PORT").unwrap();
        assert_eq!(port, 9999);
    }
    #[test]
    fn check_int() {
        let store = ConfigLoader::new(
            convert! {
                PORT:i32=>min(1000) max(10000)
            },
            None,
        )
        .unwrap();
        let port: i32 = store.get("PORT").unwrap();
        assert_eq!(port, 9999);
    }
    #[test]
    fn check_int_store_must_fail_on_constraints() {
        if ConfigLoader::new(
            convert! {
                PORT:i32=>min(1000) max(2000)
            },
            None,
        )
        .is_ok()
        {
            panic!("Store cannot be created because of constraints")
        }
    }
    #[test]
    fn check_str() {
        let store = ConfigLoader::new(
            convert! {
                HOST:str=>min(4) max(20)
            },
            None,
        )
        .unwrap();
        let host: String = store.get("HOST").unwrap();

        assert_eq!(host, "localhost");
    }
    #[test]
    fn check_str_must_fail_on_constraints() {
        if ConfigLoader::new(
            convert! {
                HOST:str=>min(4) max(8)
            },
            None,
        )
        .is_ok()
        {
            panic!("Store cannot be created because of constraints")
        }
    }
    #[test]
    fn check_bool() {
        let store = ConfigLoader::new(
            convert! {
                CRITICAL_FLAG:bool
            },
            None,
        )
        .unwrap();
        let flag: bool = store.get("CRITICAL_FLAG").unwrap();

        assert!(flag);
    }
    #[test]
    fn check_long() {
        let store = ConfigLoader::new(
            convert! {
                LONG_VAR:long
            },
            None,
        )
        .unwrap();
        let num: i64 = store.get("LONG_VAR").unwrap();

        assert_eq!(num, 5405632342349523);
    }
    #[test]
    fn check_long_with_constraint() {
        let store = ConfigLoader::new(
            convert! {
                LONG_VAR:long => min(1234912)
            },
            None,
        )
        .unwrap();
        let num: i64 = store.get("LONG_VAR").unwrap();

        assert_eq!(num, 5405632342349523);
    }

    #[test]
    fn check_multiple_values_with_macro() {
        let store = ConfigLoader::new(
            convert! {
                PORT:int => min(1) max(10000) optional,
                HOST: str => min(4) max(12) optional,
                CRITICAL_FLAG:bool,
                LONG_VAR:i64 => min(4) optional,
                OP:str=>notEmpty
            },
            None,
        )
        .unwrap();
        let port: i32 = store.get("PORT").unwrap();
        let host: String = store.get("HOST").unwrap();
        let flag: bool = store.get("CRITICAL_FLAG").unwrap();
        let num: i64 = store.get("LONG_VAR").unwrap();
        assert_eq!(port, 9999);
        assert_eq!(host, "localhost");
        assert!(flag);
        assert_eq!(num, 5405632342349523);
    }
    #[test]
    fn check_optional_store_fail() {
        let env_values = convert! {
            ASASDASD:str
        };
        if ConfigLoader::new(env_values, None).is_ok() {
            panic!("Value is not present in test")
        }
    }

    #[test]
    fn check_optional_store_not_fail() {
        let env_values = convert! {
            ASASDASD:str=>optional
        };
        if ConfigLoader::new(env_values, None).is_err() {
            panic!("Value is optional and store should be created correctly")
        }
    }
}
