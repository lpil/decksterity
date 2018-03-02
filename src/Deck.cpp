#include "Deck.h"

auto Deck::load(string path) -> void {
  player.load("techno.flac");
  player.play();
}

auto Deck::playPause() -> void {
  if (player.isPlaying()) {
    player.setPaused(true);
  } else {
    player.setPaused(false);
  }
}

auto Deck::adjustSpeed(float multiplier) -> void {
  auto newSpeed = player.getSpeed() * multiplier;
  player.setSpeed(newSpeed);

  auto percentage = floor(newSpeed * 10000) / 100;
  ofLog(OF_LOG_NOTICE) << "deck speed: " << ofToString(percentage) << "%";
}
