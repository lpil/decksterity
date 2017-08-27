module UniverseSpec
  ( main
  , spec
  ) where

import Test.Hspec
import Test.Hspec.Runner

main :: IO ()
main = hspecWith defaultConfig {configFastFail = True} spec

spec :: Spec
spec =
  describe "the Universe" $ do
    it "behaves the way we expect" $ 1 + 1 `shouldBe` 2
    it "hasn't changes since we last checked" $ 10 - 5 `shouldBe` 5
