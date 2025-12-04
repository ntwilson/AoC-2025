module Test.Main where

import Prelude

import Data.Either (Either(..))
import Effect (Effect)
import Effect.Aff (runAff_)
import Effect.Class (liftEffect)
import Effect.Class.Console (log)
import Test.Spec.Discovery (discover)
import Test.Spec.Reporter (consoleReporter)
import Test.Spec.Runner.Node (runSpecAndExitProcess)

main :: Effect Unit
main = runAff_ handler do
  tests <- discover "Test\\..*"
  liftEffect $ runSpecAndExitProcess [ consoleReporter ] tests

  where
  handler (Left e) = log ("Unable to start test run. Error: " <> show e)
  handler (Right _) = pure unit