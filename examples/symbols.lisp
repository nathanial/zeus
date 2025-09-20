; Symbol demonstration file
; Shows the new symbol features in Zeus Lisp

(println "=== Zeus Lisp Symbol Features Demo ===")
(println "")

; Keywords - self-evaluating symbols
(println "1. Keywords (self-evaluating):")
(println "   :keyword evaluates to:")
(println :keyword)
(println "   :another-keyword evaluates to:")
(println :another-keyword)
(println "")

; Keywords can be stored in variables
(println "2. Keywords in variables:")
(define status :active)
(println "   status =")
(println status)
(println "")

; Gensym - generate unique uninterned symbols
(println "3. Gensym (unique symbols):")
(println "   (gensym) generates:")
(println (gensym))
(println "   (gensym) again generates:")
(println (gensym))
(println "   (gensym \"temp\") generates:")
(println (gensym "temp"))
(println "")

; Symbol properties - associate metadata with symbols
(println "4. Symbol Properties:")
(put (quote person) (quote name) "Alice")
(put (quote person) (quote age) 30)
(put (quote person) (quote city) "New York")
(println "   Properties of 'person:")
(println "   name:")
(println (get (quote person) (quote name)))
(println "   age:")
(println (get (quote person) (quote age)))
(println "   city:")
(println (get (quote person) (quote city)))
(println "")

; Property list
(println "5. Symbol Property List:")
(println "   Full property list of 'person:")
(println (symbol-plist (quote person)))
(println "")

; Using keywords in case statements
(println "6. Keywords in case statements:")
(define color :red)
(case color
  (:red (println "   The color is red"))
  (:blue (println "   The color is blue"))
  (:green (println "   The color is green"))
  (else (println "   Unknown color")))
(println "")

; Demonstrating keyword equality
(println "7. Keyword equality:")
(if (= :test :test)
    (println "   :test equals :test")
    (println "   :test does not equal :test"))
(if (= :foo :bar)
    (println "   :foo equals :bar")
    (println "   :foo does not equal :bar"))
(println "")

(println "=== Demo Complete ===")