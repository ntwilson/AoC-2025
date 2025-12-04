module Main where

import Prelude

import Day1 (day1)
import Effect (Effect)
import Effect.Aff (launchAff_)

main :: Effect Unit
main = launchAff_ do
  day1

