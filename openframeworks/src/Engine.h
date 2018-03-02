#pragma once

#include "ofSoundPlayer.h"

class Engine {
public:
  enum Deck { A, B };

  auto load(Deck deck, string path) -> void;
  auto playPause(Deck deck) -> void;
  auto adjustSpeed(Deck deck, float delta) -> void;

private:
  ofSoundPlayer playerA;
  ofSoundPlayer playerB;

  auto getPlayer(Deck d) -> ofSoundPlayer;
  auto deckString(Deck d) -> std::string;
};
