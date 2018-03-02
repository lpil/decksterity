#include "ofApp.h"

//--------------------------------------------------------------
auto ofApp::setup() -> void { deck.load("techno.flac"); }

//--------------------------------------------------------------
auto ofApp::update() -> void {}

//--------------------------------------------------------------
auto ofApp::draw() -> void {}

//--------------------------------------------------------------
auto ofApp::keyPressed(int key) -> void {
  switch (key) {
  case ' ':
    deck.playPause();
    break;

  case OF_KEY_UP:
    deck.adjustSpeed(1.001);
    break;

  case OF_KEY_DOWN:
    deck.adjustSpeed(0.999);
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
