#include "ofApp.h"

//--------------------------------------------------------------
auto ofApp::setup() -> void {
  // engine.load(Engine::A, "techno.flac");
  // engine.load(Engine::B, "techno.flac");

  ofSetFrameRate(60);

  sampleRate = 44100; /* Sampling Rate */
  bufferSize = 512;   /* Buffer Size. you have to fill this buffer with sound
                         using the for loop in the audioOut method */

  ofxMaxiSettings::setup(sampleRate, 2, bufferSize);

  ofSetVerticalSync(true);
  ofEnableAlphaBlending();
  ofEnableSmoothing();

  // Anything that you would normally find/put in maximilian's setup() method
  // needs to go here. For example, Sample loading.

  mySample.load(ofToDataPath("jungle.flac"));

  ofBackground(0, 0, 0);

  // this has to happen at the end of setup - it switches on the DAC
  ofSoundStreamSetup(2, 2, this, sampleRate, bufferSize, 4);
}

//--------------------------------------------------------------
auto ofApp::update() -> void {}

//--------------------------------------------------------------
auto ofApp::draw() -> void {}

//--------------------------------------------------------------
auto ofApp::keyPressed(int key) -> void {
  // switch (key) {
  // // Deck A
  // case 'q':
  //   engine.adjustSpeed(Engine::A, 1.001);
  //   break;

  // case 'a':
  //   engine.adjustSpeed(Engine::A, 0.999);
  //   break;

  // case 'z':
  //   engine.playPause(Engine::A);
  //   break;

  // // Deck B
  // case 'w':
  //   engine.adjustSpeed(Engine::B, 1.001);
  //   break;

  // case 's':
  //   engine.adjustSpeed(Engine::B, 0.999);
  //   break;

  // case 'x':
  //   engine.playPause(Engine::B);
  //   break;
  // }
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

//--------------------------------------------------------------
void ofApp::audioOut(float *output, int bufferSize, int nChannels) {

  // for (int i = 0; i < bufferSize; i++) {

    /* Stick your maximilian 'play()' code in here ! Declare your objects in
     testApp.h.

     For information on how maximilian works, take a look at the example code at

     http://www.maximilian.strangeloop.co.uk

     under 'Tutorials'.

     */

    // make 'wave' equal something noisy

    /* You may end up with lots of outputs. add them here */
    auto buffer = mySample.play(1);
    output[i * nChannels] = buffer;
    output[i * nChannels + 1] = buffer;
  // }
}

//--------------------------------------------------------------
void ofApp::audioIn(float *input, int bufferSize, int nChannels) {

  for (int i = 0; i < bufferSize; i++) {
    /* you can also grab the data out of the arrays*/
  }
}
