open Belt;

[@bs.deriving abstract]
type t = {
  title: string,
  artist: option(string),
  album: option(string),
  bpm: option(int),
  number: option(int),
};

let artist = track => track |> artist |> Option.getWithDefault(_, "Unknown Artist");

/*
  Example data
 */
let trackA: option(t) =
  Some(t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ));

let trackB: option(t) = None;


let tracks = [
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
  t(
    ~title="Rip your nips off",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180),
    ~number=Some(1),
  ),
];
