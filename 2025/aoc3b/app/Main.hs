module Main where
import Debug.Trace (trace)

main :: IO ()
main = do 
  contents <- readFile "input.txt"
  let output = foldl addBatteryBank 0 (lines contents)
  print output


addBatteryBank :: Int -> String -> Int
addBatteryBank acc val = 
  let batteries = largestBatteryCombination val
  in acc + batteries

largestBatteryCombination :: String -> Int 
largestBatteryCombination val = 
  let numbers = trace ("Input val: " ++ val) $ map toInt val 
  in buildNumber numbers 12 0 0

buildNumber :: [Int] -> Int -> Int -> Int -> Int
buildNumber _ 0 _ acc = acc
buildNumber numbers remaining lastIndex acc = 
  let offset = 12 - remaining
      dropAmount = lastIndex + offset
      dropLastAmount = remaining - 1
      availableLength = length numbers - dropAmount - dropLastAmount

      searchList = take availableLength (drop dropAmount numbers)
      (value, index) = maxNum searchList
      newAcc = acc * 10 + value
  in buildNumber numbers (remaining - 1) (index + lastIndex) newAcc

toInt :: Char -> Int
toInt a = read [a]

dropLast :: Int -> [a] -> [a]
dropLast n xs = take (length xs - n) xs

maxNum :: Ord a => [a] -> (a, Int)
maxNum [] = error "empty list"
maxNum (x:xs) = maxNumHelper xs 1 0 x
  where
    maxNumHelper [] _ max_index max_val = (max_val, max_index)
    maxNumHelper (y:ys) current_index max_index max_val =
        if y > max_val
        then maxNumHelper ys (current_index + 1) current_index y
        else maxNumHelper ys (current_index + 1) max_index max_val

