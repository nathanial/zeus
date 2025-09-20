; Factorial example
; Demonstrates recursive functions in Zeus Lisp

(println "=== Factorial Calculator ===")
(println "")

; Define factorial function recursively
(defun factorial (n)
  (if (< n 2)
      1
      (* n (factorial (- n 1)))))

; Test factorial with various inputs
(println "factorial(0) =")
(println (factorial 0))

(println "factorial(1) =")
(println (factorial 1))

(println "factorial(5) =")
(println (factorial 5))

(println "factorial(10) =")
(println (factorial 10))

(println "")

; Define an iterative version using do
(defun factorial-iter (n)
  (do ((i 1 (+ i 1))
       (result 1 (* result i)))
      ((> i n) result)))

(println "Iterative version:")
(println "factorial-iter(5) =")
(println (factorial-iter 5))

(println "factorial-iter(10) =")
(println (factorial-iter 10))

(println "")
(println "=== Complete ===")