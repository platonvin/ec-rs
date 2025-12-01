#![allow(unused)]

#[cfg(test)]
mod helper_macro_tests {
    use ecs::*;

    #[test]
    fn test_token_match() {
        assert!(ident_eq!(Foo, Foo, { true }, { false }));
        assert!(!ident_eq!(Foo, Bar, { true }, { false }));
        assert!(!ident_eq!(Bar, Foo, { true }, { false }));
    }

    #[test]
    fn test_if_has_type() {
        let mut result;

        result = false;
        if_has_type!(A; A, B, C; { result = true; } {});
        assert!(result);

        result = false;
        if_has_type!(B; A, B, C; { result = true; } {});
        assert!(result);

        result = false;
        if_has_type!(C; A, B, C; { result = true; } {});
        assert!(result);

        result = false;
        if_has_type!(D; A, B, C; { result = true; } {});
        assert!(!result);

        result = false;
        if_has_type!(X; X; { result = true; } {});
        assert!(result);

        result = false;
        if_has_type!(X; ; { result = true; } {});
        assert!(!result);

        result = false;
        if_has_type!(Y; X, Z; { result = true; } {});
        assert!(!result);
    }

    #[test]
    fn test_if_all_present() {
        let mut result;

        result = false;
        if_all_present!((A, B); ; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((); ; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((Position, Velocity); Position; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((Position, Velocity); Health; { result = true; } {});
        assert!(!result);

        result = false;
        if_all_present!((Position, Velocity); Position, Velocity; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((Position, Velocity); Velocity, Position; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((Position, Velocity, Health); Position, Velocity; { result = true; } {});
        assert!(result);

        result = false;
        if_all_present!((Position, Health); Position, Velocity; { result = true; } {});
        assert!(!result);

        result = false;
        if_all_present!((Position); Position, Velocity; { result = true; } {});
        assert!(!result);
    }
}
