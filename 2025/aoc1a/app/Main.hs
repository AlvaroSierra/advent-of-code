import Data.Char (digitToInt)

main :: IO ()
main = do
    contents <- readFile "input.txt"
    let output = foldl add (50, 0, 0) (lines contents)
    print output

add :: (Integer, Integer, Integer) -> String -> (Integer, Integer, Integer)
add (acc, count_partb, count_parta) y = 
    let steps = parse y
        delta = if steps >= 0 then 1 else -1
        (newAcc, new_count_partb) = iterate (step delta) (acc, count_partb) !! abs (fromInteger steps)
        countAAdd = if newAcc == 0 then 1 else 0
    in (newAcc, new_count_partb, count_parta + countAAdd)

step :: Integer -> (Integer, Integer) -> (Integer, Integer)
step delta (acc, count_partb) =
    let newAcc = mod (acc + delta) 100
        partBAdd = if newAcc == 0 then 1 else 0
    in (newAcc, count_partb + partBAdd)

parse :: String -> Integer
parse (dir:digits) = multiplier * fromDigits digits
  where multiplier = if dir == 'L' then -1 else 1

fromDigits :: String -> Integer
fromDigits = foldl addDigit 0
  where
    addDigit num d = 10 * num + toInteger (digitToInt d)
