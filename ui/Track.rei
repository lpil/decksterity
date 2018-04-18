type t;

/*
  Getters and setters
 */

let id: t => string;
let artist: t => string;
let title: t => string;
let album: t => option(string);
let bpm: t => option(float);
let number: t => option(int);

/*
  Example data
 */

let tracks: list(t);
let trackA: option(t);
let trackB: option(t);
