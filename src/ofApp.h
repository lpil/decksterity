#pragma once

#include "Deck.h"
#include "ofMain.h"

class ofApp : public ofBaseApp {

public:
  auto setup() -> void;
  auto update() -> void;
  auto draw() -> void;

  auto keyPressed(int key) -> void;
  auto keyReleased(int key) -> void;
  auto mouseMoved(int x, int y) -> void;
  auto mouseDragged(int x, int y, int button) -> void;
  auto mousePressed(int x, int y, int button) -> void;
  auto mouseReleased(int x, int y, int button) -> void;
  auto mouseEntered(int x, int y) -> void;
  auto mouseExited(int x, int y) -> void;
  auto windowResized(int w, int h) -> void;
  auto dragEvent(ofDragInfo dragInfo) -> void;
  auto gotMessage(ofMessage msg) -> void;

private:
  Deck deck;
};
