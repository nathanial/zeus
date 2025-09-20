; List operations demonstration
; Shows various list manipulation functions in Zeus Lisp

(println "=== List Operations Demo ===")
(println "")

; Create some lists
(define numbers (list 1 2 3 4 5))
(define fruits (list "apple" "banana" "orange"))
(define mixed (list 1 "two" 3.14 :keyword))

(println "1. Basic Lists:")
(println "   numbers:")
(println numbers)
(println "   fruits:")
(println fruits)
(println "   mixed:")
(println mixed)
(println "")

; List operations
(println "2. Basic Operations:")
(println "   (car numbers) =")
(println (car numbers))
(println "   (cdr numbers) =")
(println (cdr numbers))
(println "   (cons 0 numbers) =")
(println (cons 0 numbers))
(println "   (append numbers (list 6 7 8)) =")
(println (append numbers (list 6 7 8)))
(println "   (reverse numbers) =")
(println (reverse numbers))
(println "   (length numbers) =")
(println (length numbers))
(println "")

; Accessing elements
(println "3. Element Access:")
(println "   (nth 0 fruits) =")
(println (nth 0 fruits))
(println "   (nth 2 fruits) =")
(println (nth 2 fruits))
(println "   (nthcdr 2 numbers) =")
(println (nthcdr 2 numbers))
(println "")

; Higher-order functions
(println "4. Higher-Order Functions:")

; Map - square all numbers
(define squares (mapcar (lambda (x) (* x x)) numbers))
(println "   Squares of numbers:")
(println squares)

; Filter - get numbers greater than 2
(define greater-than-two (filter (lambda (x) (> x 2)) numbers))
(println "   Numbers greater than 2:")
(println greater-than-two)

; Reduce - sum all numbers
(define sum (reduce + numbers))
(println "   Sum of numbers:")
(println sum)

; Remove - remove all 2s
(define no-twos (remove (lambda (x) (= x 2)) numbers))
(println "   Numbers without 2:")
(println no-twos)
(println "")

; Member testing
(println "5. Membership Testing:")
(println "   Is 3 in numbers?")
(if (member 3 numbers)
    (println "   Yes")
    (println "   No"))
(println "   Is 10 in numbers?")
(if (member 10 numbers)
    (println "   Yes")
    (println "   No"))
(println "")

(println "=== Demo Complete ===")