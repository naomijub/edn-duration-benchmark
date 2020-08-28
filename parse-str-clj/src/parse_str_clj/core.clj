(ns parse-str-clj.core
  (:require [criterium.core :refer [bench]])
  (:gen-class))

(def edn
  "{
        :type :human
        :first-name \"bench\"
        :last-name \"mark\"
        :age 13
        :version 0.13
        :associates [
            {
                :name :julia
                :role :adm
            }
            {
                :name :otavio
                :role :contributor
            }
            {
                :name :juxt
                :role :great-ideas
            }
        ]
    }")

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  ;; warmup here
  (let [_     (time (clojure.edn/read-string edn))
        parsed_edn   (clojure.edn/read-string edn)
        _ (time (-> parsed_edn :associates (get 0) :role))]
    (bench (clojure.edn/read-string edn))
    (bench (-> parsed_edn :associates (get 0) :role))))
