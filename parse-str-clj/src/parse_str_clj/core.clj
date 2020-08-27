(ns parse-str-clj.core
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
  (let [duration     (time (clojure.edn/read-string edn))
        parsed_edn   (clojure.edn/read-string edn)
        duration_nav (time (get-in parsed_edn  [:associates 0 :role]))]
    (println duration)
    (println duration_nav)))
