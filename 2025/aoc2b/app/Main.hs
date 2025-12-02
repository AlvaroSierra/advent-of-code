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
  let nDigits = numDigits num
  in if iterateUntilJust [1..(div nDigits 2)] num checkSingleNumberLevel then acc + num else acc

checkSingleNumberLevel :: (Int, Int) -> Bool
checkSingleNumberLevel (digitsToDuplicate, number) = 
  let nDigitsFitsNumber = mod (numDigits number) digitsToDuplicate == 0 
      chunks = chunk digitsToDuplicate (show number)
  in nDigitsFitsNumber && allEqual chunks

chunk :: Int -> [a] -> [[a]]
chunk _ [] = []
chunk n xs = take n xs : chunk n (drop n xs)

allEqual :: Eq a => [[a]] -> Bool
allEqual [] = True
allEqual (x:xs) = all (== x) xs

iterateUntilJust :: [a] -> Int -> ((a, Int) -> Bool) -> Bool
iterateUntilJust [] _ _ = False
iterateUntilJust (x:xs) num f =
  f (x, num) || iterateUntilJust xs num f

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
