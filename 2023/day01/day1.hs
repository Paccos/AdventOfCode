import Data.Char (digitToInt, isNumber)
import Data.List (sort, sortOn)
import Data.Maybe (mapMaybe)
import Data.Text qualified as T
import Data.Text.Internal.Search (indices)

numStrsAndInts :: [(String, Int)]
numStrsAndInts = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

indicesOfNumberStrAndInt :: String -> (String, Int) -> (Int, [Int])
indicesOfNumberStrAndInt searchStr (str, num) = (num, sort $ indices (T.pack str) (T.pack searchStr) ++ indices (T.pack (show num)) (T.pack searchStr))

indicesOfNumbers :: String -> [(Int, [Int])]
indicesOfNumbers str = map (indicesOfNumberStrAndInt str) numStrsAndInts

minAndMaxIndexOfNum :: (Int, [Int]) -> Maybe (Int, Int, Int)
minAndMaxIndexOfNum (num, idxs) = case (num, idxs) of
  (_, []) -> Nothing
  (num, idxs) -> Just (num, minimum idxs, maximum idxs)

minAndMaxIndicesOfNumbers :: [(Int, [Int])] -> [(Int, Int, Int)]
minAndMaxIndicesOfNumbers = mapMaybe minAndMaxIndexOfNum

firstNumber :: [(Int, Int, Int)] -> Int
firstNumber xs = fst $ head $ sortOn snd $ map (\(a, b, c) -> (a, b)) xs

lastNumber :: [(Int, Int, Int)] -> Int
lastNumber xs = fst $ last $ sortOn snd $ map (\(a, b, c) -> (a, c)) xs

firstAndLastConcatenated :: [Int] -> Int
firstAndLastConcatenated xs = head xs * 10 + last xs

firstAndLastConcatenated' :: [(Int, [Int])] -> Int
firstAndLastConcatenated' xs = firstNumber (minAndMaxIndicesOfNumbers xs) * 10 + lastNumber (minAndMaxIndicesOfNumbers xs)

numbers :: String -> [Int]
numbers str = map digitToInt $ filter isNumber str

main = do
  input <- readFile "input"
  let inputLines = lines input
  let linesToNumbers = map indicesOfNumbers inputLines
  let firstAndListCiphers = map firstAndLastConcatenated' linesToNumbers
  let result = sum firstAndListCiphers
  print result
