module Day1 where

import Prelude

import Data.Array as Array
import Data.Either (Either(..))
import Data.Foldable (sum)
import Data.Int as Int
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..))
import Data.String as String
import Data.String.CodePoints (codePointFromChar)
import Data.Traversable (traverse)
import Effect.Aff (Aff, throwError)
import Effect.Class (liftEffect)
import Effect.Class.Console (log)
import Effect.Exception (throw)
import Node.Encoding (Encoding(..))
import Node.FS.Aff (readTextFile)

getInputs :: Aff (Array String)
getInputs = do
  contents <- readTextFile UTF8 "../puzzleInput/day1.txt"
  pure $ Array.filter (not String.null) $ String.split (Pattern "\n") contents

data Input = L Int | R Int

parseInput :: String -> Either String Input
parseInput input = 
  case String.uncons input of 
    Just { head, tail } 
      | head == codePointFromChar 'R', Just movement <- Int.fromString tail -> pure $ R movement
      | head == codePointFromChar 'L', Just movement <- Int.fromString tail -> pure $ L movement
    _ -> throwError ("Unable to parse " <> input <> " as a valid input.")


solvePuzzle1 :: Array Input -> Int 
solvePuzzle1 inputs =
  let 
    startingPos = 50
    endingSpots = 
      Array.scanl
        (\state element -> 
          case element of 
            L n -> (state - n) `mod` 100
            R n -> (state + n) `mod` 100
        ) 
        startingPos 
        inputs
  in
    endingSpots # Array.filter (_ == 0) # Array.length

solvePuzzle2 :: Array Input -> Int
solvePuzzle2 inputs =
  let 
    startingPos = 50

    zeroTraversals = 
      Array.scanl
        (\{acc} element ->
          let 
            zeros = case element of
              R n -> (n + acc) / 100
              L n -> max (n - acc + (if acc == 0 then 0 else 100)) 0 / 100
           

            nextAcc = case element of
              R n -> (acc + n) `mod` 100
              L n -> (acc - n) `mod` 100

          in 
            { acc: nextAcc, count: zeros }
        )
        { acc: startingPos, count: 0 }
        inputs

  in
    zeroTraversals <#> _.count # sum

day1 :: Aff Unit
day1 = do
  inputsRaw <- getInputs
  inputs <- 
    case traverse parseInput inputsRaw of
      Left err -> liftEffect $ throw err
      Right vals -> pure vals

  let 
    puzzle1Solution = solvePuzzle1 inputs
    puzzle2Solution = solvePuzzle2 inputs

  log $ "Day 1, Puzzle 1 Solution: " <> show puzzle1Solution
  log $ "Day 1, Puzzle 2 Solution: " <> show puzzle2Solution
