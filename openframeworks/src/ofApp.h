#pragma once

#include "Engine.h"
#include "ofMain.h"
#include "ofxMaxim.h"

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

  auto audioOut(float *output, int bufferSize, int nChannels) -> void;
  auto audioIn(float *input, int bufferSize, int nChannels) -> void;

  int bufferSize; /* buffer size */
  int sampleRate;

  maxiSample mySample;

private:
  // Engine engine;
};
