#include "Engine.h"

auto Engine::load(Deck deck, string path) -> void {
  auto loaded = getPlayer(deck).load(path);

  ofLog(OF_LOG_NOTICE) << "deck " << deckString(deck) << " load " << path
                       << (loaded ? " success" : " fail");
}

auto Engine::playPause(Deck deck) -> void {
  auto player = getPlayer(deck);
  player.setPaused(player.isPlaying());

  ofLog(OF_LOG_NOTICE) << "deck " << deckString(deck)
                       << (player.isPlaying() ? " paused" : " playing");
}

auto Engine::adjustSpeed(Deck deck, float multiplier) -> void {
  auto player = getPlayer(A);
  auto newSpeed = player.getSpeed() * multiplier;
  player.setSpeed(newSpeed);

  auto percentage = floor(newSpeed * 10000) / 100;
  ofLog(OF_LOG_NOTICE) << "deck " << deckString(deck)
                       << " speed: " << ofToString(percentage) << "%";
}

auto Engine::getPlayer(Deck deck) -> ofSoundPlayer {
  switch (deck) {
  case A:
    return playerA;
  default:
    return playerB;
  }
};

auto Engine::deckString(Deck deck) -> std::string {
  switch (deck) {
  case A:
    return "A";
  default:
    return "B";
  }
};
