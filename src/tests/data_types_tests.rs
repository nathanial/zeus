#[cfg(test)]
mod data_types_tests {
    use crate::interpreter::evaluator::Evaluator;

    #[test]
    fn test_integers() {
        let mut evaluator = Evaluator::new();

        // Basic integer literals
        assert_eq!(
            evaluator.eval_str("42").unwrap(),
            evaluator.eval_str("42").unwrap()
        );

        // Integer arithmetic preserves integer type
        assert_eq!(
            evaluator.eval_str("(+ 1 2 3)").unwrap(),
            evaluator.eval_str("6").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(- 10 5)").unwrap(),
            evaluator.eval_str("5").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(* 3 4)").unwrap(),
            evaluator.eval_str("12").unwrap()
        );

        // Integer predicate
        assert_eq!(
            evaluator.eval_str("(integerp 42)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(integerp 3.14)").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_floats() {
        let mut evaluator = Evaluator::new();

        // Float literals
        assert!(evaluator.eval_str("3.14").is_ok());

        // Mixed arithmetic produces floats
        assert_eq!(
            evaluator.eval_str("(+ 1 2.5)").unwrap(),
            evaluator.eval_str("3.5").unwrap()
        );

        // Division always produces float
        assert!(evaluator.eval_str("(/ 10 3)").is_ok());

        // Float predicate
        assert_eq!(
            evaluator.eval_str("(floatp 3.14)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(floatp 42)").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_characters() {
        let mut evaluator = Evaluator::new();

        // Character literals
        assert!(evaluator.eval_str("#\\a").is_ok());
        assert!(evaluator.eval_str("#\\space").is_ok());
        assert!(evaluator.eval_str("#\\newline").is_ok());

        // Character comparison
        assert_eq!(
            evaluator.eval_str("(char= #\\a #\\a)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(char= #\\a #\\b)").unwrap(),
            Evaluator::bool_to_expr(false)
        );

        assert_eq!(
            evaluator.eval_str("(char< #\\a #\\b)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        // Character conversion
        assert_eq!(
            evaluator.eval_str("(char->integer #\\A)").unwrap(),
            evaluator.eval_str("65").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(integer->char 65)").unwrap(),
            evaluator.eval_str("#\\A").unwrap()
        );

        // Character predicate
        assert_eq!(
            evaluator.eval_str("(characterp #\\a)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(characterp \"a\")").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_vectors() {
        let mut evaluator = Evaluator::new();

        // Vector literals
        assert!(evaluator.eval_str("[1 2 3]").is_ok());
        assert!(evaluator.eval_str("[]").is_ok());

        // Vector construction
        assert!(evaluator.eval_str("(vector 1 2 3)").is_ok());
        assert!(evaluator.eval_str("(make-vector 5)").is_ok());
        assert!(evaluator.eval_str("(make-vector 3 \"hello\")").is_ok());

        // Vector operations
        let setup = "(define v [10 20 30])";
        evaluator.eval_str(setup).unwrap();

        assert_eq!(
            evaluator.eval_str("(vector-ref v 1)").unwrap(),
            evaluator.eval_str("20").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(vector-length v)").unwrap(),
            evaluator.eval_str("3").unwrap()
        );

        // Vector modification
        evaluator
            .eval_str("(define v2 (vector-set! v 1 99))")
            .unwrap();
        assert_eq!(
            evaluator.eval_str("(vector-ref v2 1)").unwrap(),
            evaluator.eval_str("99").unwrap()
        );

        // Vector predicate
        assert_eq!(
            evaluator.eval_str("(vectorp [1 2 3])").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(vectorp (list 1 2 3))").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_hash_tables() {
        let mut evaluator = Evaluator::new();

        // Hash table creation
        evaluator.eval_str("(define h (make-hash-table))").unwrap();

        // Hash table operations
        evaluator
            .eval_str("(define h2 (hash-set! h \"key1\" 100))")
            .unwrap();
        evaluator
            .eval_str("(define h3 (hash-set! h2 \"key2\" 200))")
            .unwrap();
        evaluator
            .eval_str("(define h4 (hash-set! h3 :keyword 300))")
            .unwrap();

        assert_eq!(
            evaluator.eval_str("(hash-ref h4 \"key1\")").unwrap(),
            evaluator.eval_str("100").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(hash-ref h4 \"key2\")").unwrap(),
            evaluator.eval_str("200").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(hash-ref h4 :keyword)").unwrap(),
            evaluator.eval_str("300").unwrap()
        );

        // Default value for missing key
        assert_eq!(
            evaluator
                .eval_str("(hash-ref h4 \"missing\" \"default\")")
                .unwrap(),
            evaluator.eval_str("\"default\"").unwrap()
        );

        // Hash removal
        evaluator
            .eval_str("(define h5 (hash-remove! h4 \"key1\"))")
            .unwrap();
        assert!(evaluator.eval_str("(hash-ref h5 \"key1\")").is_err());

        // Hash keys
        assert!(evaluator.eval_str("(hash-keys h4)").is_ok());

        // Hash table predicate
        assert_eq!(
            evaluator.eval_str("(hash-table-p h)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(hash-table-p (list 1 2))").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_mixed_data_types() {
        let mut evaluator = Evaluator::new();

        // Vectors can contain any type
        evaluator
            .eval_str("(define mixed-vec [1 \"string\" #\\a :keyword])")
            .unwrap();
        assert_eq!(
            evaluator.eval_str("(vector-ref mixed-vec 1)").unwrap(),
            evaluator.eval_str("\"string\"").unwrap()
        );

        // Hash tables with different key types
        evaluator.eval_str("(define h (make-hash-table))").unwrap();
        evaluator
            .eval_str("(define h1 (hash-set! h 42 \"integer-key\"))")
            .unwrap();
        evaluator
            .eval_str("(define h2 (hash-set! h1 \"str\" \"string-key\"))")
            .unwrap();
        evaluator
            .eval_str("(define h3 (hash-set! h2 :sym \"keyword-key\"))")
            .unwrap();

        assert_eq!(
            evaluator.eval_str("(hash-ref h3 42)").unwrap(),
            evaluator.eval_str("\"integer-key\"").unwrap()
        );

        // numberp predicate
        assert_eq!(
            evaluator.eval_str("(numberp 42)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(numberp 3.14)").unwrap(),
            Evaluator::bool_to_expr(true)
        );

        assert_eq!(
            evaluator.eval_str("(numberp \"42\")").unwrap(),
            Evaluator::bool_to_expr(false)
        );
    }

    #[test]
    fn test_vector_of_vectors() {
        let mut evaluator = Evaluator::new();

        // Nested vectors
        evaluator
            .eval_str("(define matrix [[1 2] [3 4] [5 6]])")
            .unwrap();

        assert_eq!(
            evaluator
                .eval_str("(vector-ref (vector-ref matrix 1) 0)")
                .unwrap(),
            evaluator.eval_str("3").unwrap()
        );

        assert_eq!(
            evaluator.eval_str("(vector-length matrix)").unwrap(),
            evaluator.eval_str("3").unwrap()
        );
    }

    #[test]
    fn test_hash_table_with_complex_values() {
        let mut evaluator = Evaluator::new();

        // Hash table with vector values
        evaluator.eval_str("(define h (make-hash-table))").unwrap();
        evaluator
            .eval_str("(define h1 (hash-set! h \"coords\" [10 20 30]))")
            .unwrap();

        let vec = evaluator.eval_str("(hash-ref h1 \"coords\")").unwrap();
        assert_eq!(
            evaluator
                .eval_str("(vector-ref (hash-ref h1 \"coords\") 1)")
                .unwrap(),
            evaluator.eval_str("20").unwrap()
        );

        // Hash table with list values
        evaluator
            .eval_str("(define h2 (hash-set! h1 \"list\" (list 1 2 3)))")
            .unwrap();
        assert!(evaluator.eval_str("(car (hash-ref h2 \"list\"))").is_ok());
    }

    #[test]
    fn test_type_conversion() {
        let mut evaluator = Evaluator::new();

        // Integer to float happens automatically in mixed operations
        assert!(evaluator.eval_str("(+ 1 2.5)").is_ok());

        // Character to integer and back
        assert_eq!(
            evaluator
                .eval_str("(integer->char (char->integer #\\Z))")
                .unwrap(),
            evaluator.eval_str("#\\Z").unwrap()
        );
    }
}
