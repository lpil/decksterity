#pragma once

#include "ofSoundPlayer.h"

class Deck {
public:
  auto load(string path) -> void;
  auto playPause() -> void;
  auto adjustSpeed(float delta) -> void;

private:
  ofSoundPlayer player;
};
