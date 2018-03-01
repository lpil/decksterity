#include "ofApp.h"

//--------------------------------------------------------------
void ofApp::setup() {
  deck.load("techno.flac");
  deck.play();
}

//--------------------------------------------------------------
void ofApp::update() {}

//--------------------------------------------------------------
void ofApp::draw() {}

//--------------------------------------------------------------
void ofApp::keyPressed(int key) {
  switch (key) {
  case ' ':
    playPause(deck);
    break;

  case OF_KEY_UP:
    adjustSpeed(deck, 0.05);
    break;

  case OF_KEY_DOWN:
    adjustSpeed(deck, -0.05);
    break;
  }
}

//--------------------------------------------------------------
void ofApp::keyReleased(int key) {}

//--------------------------------------------------------------
void ofApp::mouseMoved(int x, int y) {}

//--------------------------------------------------------------
void ofApp::mouseDragged(int x, int y, int button) {}

//--------------------------------------------------------------
void ofApp::mousePressed(int x, int y, int button) {}

//--------------------------------------------------------------
void ofApp::mouseReleased(int x, int y, int button) {}

//--------------------------------------------------------------
void ofApp::mouseEntered(int x, int y) {}

//--------------------------------------------------------------
void ofApp::mouseExited(int x, int y) {}

//--------------------------------------------------------------
void ofApp::windowResized(int w, int h) {}

//--------------------------------------------------------------
void ofApp::gotMessage(ofMessage msg) {}

//--------------------------------------------------------------
void ofApp::dragEvent(ofDragInfo dragInfo) {}

void ofApp::playPause(ofSoundPlayer deck) {
  if (deck.isPlaying()) {
    deck.setPaused(true);
  } else {
    deck.setPaused(false);
  }
}

void ofApp::adjustSpeed(ofSoundPlayer deck, float delta) {
  deck.setSpeed(deck.getSpeed() + delta);
}
