(ns aoc2022.past
  (:require [clojure.string :as s]))

(defn day1 []
  (let [day1-input (slurp "input/day1.txt")
        meals (map s/split-lines (s/split day1-input #"\n\n"))
        meal-sums (map #(reduce + (map read-string %)) meals)
        part1 (apply max meal-sums)
        part2 (reduce + (take 3 (sort > meal-sums)))]
    (println (str part1 "\n" part2))))

(defn day2 []
  (let [day2-input (slurp "input/day2.txt")
        day2-test-input "A Y\nB X\nC Z"
        moves (map #(s/split % #"\s") (s/split-lines day2-input))
        shape-scores {"X" 1
                      "Y" 2
                      "Z" 3}
        shape-points (map (fn [[opp you]] (get shape-scores you)) moves)
        your-moves {"X" :rock "Y" :paper "Z" :scissors}
        opp-moves {"A" :rock "B" :paper "C" :scissors}
        you-opp-matches {:rock {:rock :tie :paper :loss :scissors :win}
                         :paper {:rock :win :paper :tie :scissors :loss}
                         :scissors {:rock :loss :paper :win :scissors :tie}}
        outcome-scores {:tie 3
                        :win 6
                        :loss 0}
        outcome-points (map (fn [[opp you]]
                              (let [you (get your-moves you)
                                    opp (get opp-moves opp)]
                                (->> (get-in you-opp-matches [you opp])
                                     (get outcome-scores))))
                            moves)
        part1 (reduce + (map + shape-points outcome-points))
        howto-outcome {:rock {:tie :rock :win :paper :loss :scissors}
                       :paper {:loss :rock :tie :paper :win :scissors}
                       :scissors {:loss :paper :win :rock :tie :scissors}}
        needed-outcome {"X" :loss
                        "Y" :tie
                        "Z" :win}
        your-moves2 (map (fn [[opp you]]
                           (get-in howto-outcome [(get opp-moves opp) (get needed-outcome you)]))
                         moves)
        shape-scores2 {:rock 1
                       :paper 2
                       :scissors 3}
        shape-points2 (map #(get shape-scores2 %) your-moves2)
        outcome-points2 (map (fn [[opp you]] (get outcome-scores (get needed-outcome you))) moves)
        part2 (reduce + (map + shape-points2 outcome-points2))]
    (println (str part1 "\n" part2))))
