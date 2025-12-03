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
  let numbers = map toInt val
      (firstVal, max_index1) = maxNum (init numbers)
      second_list = drop (max_index1 + 1) numbers
      (secondVal, max_index2 ) = maxNum second_list
  in trace("val = " ++ val ++ ", firstVal = " ++ show firstVal ++ ", secondVal = " ++ show secondVal ++ ", max_index1 = " ++ show max_index1 ++ ", max_index2 = " ++ show max_index2 ++ ", second_list = " ++ show second_list) firstVal * 10 + secondVal

toInt :: Char -> Int
toInt a = read [a]

maxNum :: Ord a => [a] -> (a, Int)
maxNum [] = error "empty list"
maxNum (x:xs) = maxNumHelper xs 1 0 x
  where
    maxNumHelper [] _ max_index max_val = (max_val, max_index)
    maxNumHelper (y:ys) current_index max_index max_val =
        if y > max_val
        then maxNumHelper ys (current_index + 1) current_index y
        else maxNumHelper ys (current_index + 1) max_index max_val

