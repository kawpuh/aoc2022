(ns aoc2022.past
  (:require [clojure.string :as s]))

(defn day1 []
  (let [day1-input (slurp "input/day1.txt")
        meals (map s/split-lines (s/split day1-input #"\n\n"))
        meal-sums (map #(reduce + (map read-string %)) meals)
        part1 (apply max meal-sums)
        part2 (reduce + (take 3 (sort > meal-sums)))]
    (println (str part1 "\n" part2))))
