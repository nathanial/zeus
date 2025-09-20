#[cfg(test)]
mod symbol_tests {
    use crate::interpreter::evaluator::Evaluator;
    use crate::interpreter::types::Expr;

    #[test]
    fn test_keyword_self_evaluation() {
        let mut eval = Evaluator::new();

        // Keywords should evaluate to themselves
        let result = eval.eval_str(":foo").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Keyword(\"foo\"))");

        let result = eval.eval_str(":test-keyword").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Keyword(\"test-keyword\"))");
    }

    #[test]
    fn test_keyword_cannot_be_defined() {
        let mut eval = Evaluator::new();

        // Cannot define a keyword
        let result = eval.eval_str("(define :foo 42)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot define a keyword"));

        // Cannot use keyword as parameter
        let result = eval.eval_str("(lambda (:x) x)");
        assert!(result.is_err());
    }

    #[test]
    fn test_keyword_in_let_binding() {
        let mut eval = Evaluator::new();

        // Cannot bind to a keyword
        let result = eval.eval_str("(let ((:x 10)) x)");
        assert!(result.is_err());
        // The error message might be different, so just check it's an error
        // Could be "let binding must start with a symbol" or similar
    }

    #[test]
    fn test_gensym_basic() {
        let mut eval = Evaluator::new();

        // Basic gensym without prefix
        let result = eval.eval_str("(gensym)").unwrap();
        assert!(format!("{:?}", result).starts_with("Symbol(Uninterned(\"G"));

        // Gensym with string prefix
        let result = eval.eval_str("(gensym \"temp\")").unwrap();
        assert!(format!("{:?}", result).starts_with("Symbol(Uninterned(\"temp"));

        // Gensym with integer seed resets counter
        let result = eval.eval_str("(gensym 42)").unwrap();
        assert!(format!("{:?}", result).starts_with("Symbol(Uninterned(\"G42"));

        // Symbol argument should be rejected per Common Lisp
        assert!(eval.eval_str("(gensym (quote foo))").is_err());
    }

    #[test]
    fn test_gensym_uniqueness() {
        let mut eval = Evaluator::new();

        // Each gensym should be unique
        let sym1 = eval.eval_str("(gensym)").unwrap();
        let sym2 = eval.eval_str("(gensym)").unwrap();

        // They should not be equal
        let _result = eval.eval_str(&format!(
            "(= {} {})",
            format!("{:?}", sym1)
                .replace("Symbol(", "")
                .replace(")", ""),
            format!("{:?}", sym2)
                .replace("Symbol(", "")
                .replace(")", "")
        ));

        // Since they're uninterned, they won't be equal even with same prefix
        let sym3 = eval.eval_str("(gensym \"test\")").unwrap();
        let sym4 = eval.eval_str("(gensym \"test\")").unwrap();
        assert_ne!(format!("{:?}", sym3), format!("{:?}", sym4));
    }

    #[test]
    fn test_symbol_properties() {
        let mut eval = Evaluator::new();

        // Set a property on a symbol
        eval.eval_str("(put (quote foo) (quote color) (quote red))")
            .unwrap();

        // Get the property
        let result = eval.eval_str("(get (quote foo) (quote color))").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"red\"))");

        // Get non-existent property returns nil (empty list)
        let result = eval.eval_str("(get (quote foo) (quote size))").unwrap();
        assert_eq!(format!("{:?}", result), "List([])");

        // Set multiple properties
        eval.eval_str("(put (quote foo) (quote size) 42)").unwrap();
        eval.eval_str("(put (quote foo) (quote shape) (quote circle))")
            .unwrap();

        // Get multiple properties
        let result = eval.eval_str("(get (quote foo) (quote size))").unwrap();
        assert_eq!(result, Expr::Integer(42));

        let result = eval.eval_str("(get (quote foo) (quote shape))").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"circle\"))");
    }

    #[test]
    fn test_symbol_plist() {
        let mut eval = Evaluator::new();

        // Empty plist initially
        let result = eval.eval_str("(symbol-plist (quote bar))").unwrap();
        assert_eq!(format!("{:?}", result), "List([])");

        // Set some properties
        eval.eval_str("(put (quote bar) (quote x) 10)").unwrap();
        eval.eval_str("(put (quote bar) (quote y) 20)").unwrap();

        // Get the property list
        let result = eval.eval_str("(symbol-plist (quote bar))").unwrap();
        // The plist should contain keyword:value pairs
        assert!(format!("{:?}", result).contains("Keyword"));
        assert!(format!("{:?}", result).contains("10"));
        assert!(format!("{:?}", result).contains("20"));
    }

    #[test]
    fn test_keyword_in_case() {
        let mut eval = Evaluator::new();

        // Keywords can be used in case statements
        let result = eval
            .eval_str(
                "
            (case :foo
                (:foo (quote matched-foo))
                (:bar (quote matched-bar))
                (else (quote no-match)))
        ",
            )
            .unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"matched-foo\"))");

        // Test with variable holding keyword
        eval.eval_str("(define key :bar)").unwrap();
        let result = eval
            .eval_str(
                "
            (case key
                (:foo (quote matched-foo))
                (:bar (quote matched-bar))
                (else (quote no-match)))
        ",
            )
            .unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"matched-bar\"))");
    }

    #[test]
    fn test_keyword_equality() {
        let mut eval = Evaluator::new();

        // Keywords with same name should be equal
        let result = eval.eval_str("(= :test :test)").unwrap();
        assert_eq!(result, Evaluator::bool_to_expr(true));

        // Different keywords should not be equal
        let result = eval.eval_str("(= :foo :bar)").unwrap();
        assert_eq!(result, Expr::List(vec![]));
    }

    #[test]
    fn test_symbol_with_properties_in_list() {
        let mut eval = Evaluator::new();

        // Create a list with symbols and set properties
        eval.eval_str("(define symbols (list (quote a) (quote b) (quote c)))")
            .unwrap();
        eval.eval_str("(put (quote a) (quote value) 1)").unwrap();
        eval.eval_str("(put (quote b) (quote value) 2)").unwrap();
        eval.eval_str("(put (quote c) (quote value) 3)").unwrap();

        // Map over symbols and get their values
        let result = eval
            .eval_str(
                "
            (mapcar (lambda (sym) (get sym (quote value))) symbols)
        ",
            )
            .unwrap();
        assert_eq!(
            format!("{:?}", result),
            "List([Integer(1), Integer(2), Integer(3)])"
        );
    }

    #[test]
    fn test_property_overwrite() {
        let mut eval = Evaluator::new();

        // Set a property
        eval.eval_str("(put (quote test) (quote prop) (quote old))")
            .unwrap();
        let result = eval.eval_str("(get (quote test) (quote prop))").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"old\"))");

        // Overwrite the property
        eval.eval_str("(put (quote test) (quote prop) (quote new))")
            .unwrap();
        let result = eval.eval_str("(get (quote test) (quote prop))").unwrap();
        assert_eq!(format!("{:?}", result), "Symbol(Interned(\"new\"))");
    }
}
