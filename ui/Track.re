open Belt;

type t = {
  title: string,
  artist: option(string),
  album: option(string),
  bpm: option(int),
  number: option(int),
};

let artist = track => Option.getWithDefault(track.artist, "Unknown Artist");
let title = track => track.title;

/*
  Example data
 */
let trackA: option(t) =
  Some({
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  });

let trackB: option(t) = None;


let tracks = [
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
  {
    title: "Rip your nips off",
    artist: Some("Captain Credible"),
    album: Some("Fantasy Mansion"),
    bpm: Some(180),
    number: Some(1),
  },
];
