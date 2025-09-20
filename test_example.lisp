; Example LISP file to test file tree loading
(define factorial
  (lambda (n)
    (if (<= n 1)
        1
        (* n (factorial (- n 1))))))

(define fibonacci
  (lambda (n)
    (cond
      ((= n 0) 0)
      ((= n 1) 1)
      (else (+ (fibonacci (- n 1))
               (fibonacci (- n 2)))))))

; Test the functions
(println "Factorial of 5:" (factorial 5))
(println "Fibonacci of 7:" (fibonacci 7))