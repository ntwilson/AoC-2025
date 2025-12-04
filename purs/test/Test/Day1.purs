module Test.Day1 where

import Prelude

import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec = do
  describe "Day 1" do
    describe "modulus" do
      it "works as I expect" do
        (0 `mod` 100) `shouldEqual` 0
        (99 `mod` 100) `shouldEqual` 99
        (100 `mod` 100) `shouldEqual` 0
        (-1 `mod` 100) `shouldEqual` 99
        (-100 `mod` 100) `shouldEqual` 0
        (-101 `mod` 100) `shouldEqual` 99