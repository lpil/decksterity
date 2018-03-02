#include "ofApp.h"

//--------------------------------------------------------------
auto ofApp::setup() -> void {
  engine.load(Engine::A, "techno.flac");
  engine.load(Engine::B, "techno.flac");
}

//--------------------------------------------------------------
auto ofApp::update() -> void {}

//--------------------------------------------------------------
auto ofApp::draw() -> void {}

//--------------------------------------------------------------
auto ofApp::keyPressed(int key) -> void {
  switch (key) {
  // Deck A
  case 'q':
    engine.adjustSpeed(Engine::A, 1.001);
    break;

  case 'a':
    engine.adjustSpeed(Engine::A, 0.999);
    break;

  case 'z':
    engine.playPause(Engine::A);
    break;

  // Deck B
  case 'w':
    engine.adjustSpeed(Engine::B, 1.001);
    break;

  case 's':
    engine.adjustSpeed(Engine::B, 0.999);
    break;

  case 'x':
    engine.playPause(Engine::B);
    break;
  }
}

//--------------------------------------------------------------
auto ofApp::keyReleased(int key) -> void {}

//--------------------------------------------------------------
auto ofApp::mouseMoved(int x, int y) -> void {}

//--------------------------------------------------------------
auto ofApp::mouseDragged(int x, int y, int button) -> void {}

//--------------------------------------------------------------
auto ofApp::mousePressed(int x, int y, int button) -> void {}

//--------------------------------------------------------------
auto ofApp::mouseReleased(int x, int y, int button) -> void {}

//--------------------------------------------------------------
auto ofApp::mouseEntered(int x, int y) -> void {}

//--------------------------------------------------------------
auto ofApp::mouseExited(int x, int y) -> void {}

//--------------------------------------------------------------
auto ofApp::windowResized(int w, int h) -> void {}

//--------------------------------------------------------------
auto ofApp::gotMessage(ofMessage msg) -> void {}

//--------------------------------------------------------------
auto ofApp::dragEvent(ofDragInfo dragInfo) -> void {}
