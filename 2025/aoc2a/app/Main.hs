module Main where
import Data.Char (digitToInt)

main :: IO ()
main = do 
  contents <- readFile "input.txt"
  let cleaned_str = reverse . dropWhile (=='\n') . reverse
  let output = foldl addSolutionFromRange 0 ( splitWord (==',') (cleaned_str contents) )
  print output

addSolutionFromRange :: Int -> String -> Int
addSolutionFromRange acc curr = 
  let (low_bound, high_bound) = splitOnce '-' curr
      (low_bound_int, high_bound_int) = (fromDigits low_bound, fromDigits high_bound) 
  in foldl checkSingleNumber acc [low_bound_int..high_bound_int]



checkSingleNumber :: Int -> Int -> Int
checkSingleNumber acc num = 
  -- trace ("num = " ++ show num ++ ", acc = " ++ show acc) $ 
  let nDigits = numDigits num
      firstHalf = div num (10 ^ div nDigits 2) 
      firstHalfZeroPadded = firstHalf * (10 ^ div nDigits 2)
      isDuplicate = firstHalf == num - firstHalfZeroPadded
  in if even nDigits && isDuplicate then acc + num else acc

numDigits :: Int -> Int
numDigits n
    | n == 0    = 1
    | n < 0     = numDigits (-n)
    | otherwise = floor (logBase 10 (fromIntegral n)) + 1

splitWord :: (Char -> Bool) -> String -> [String]
splitWord separator str = case dropWhile separator str of 
  "" -> []
  str' -> w : splitWord separator str''
          where (w, str'') = break separator str'

splitOnce :: Char -> String -> (String, String)
splitOnce c str = 
    let (before, after) = break (== c) str
    in (before, drop 1 after) 

fromDigits :: String -> Int
fromDigits = foldl addDigit 0
  where
    addDigit num d = 10 * num + digitToInt d
