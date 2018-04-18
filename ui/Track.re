open Belt;

[@bs.deriving abstract]
type t = {
  title: string,
  artist: option(string),
  album: option(string),
  bpm: option(float),
  number: option(int),
};

let artist = track =>
  track |> artist |> Option.getWithDefault(_, "Unknown Artist");

let id = track => artist(track) ++ " - " ++ title(track);

/*
  Example data
 */
let trackA: option(t) =
  Some(
    t(
      ~title="Rip your nips off",
      ~artist=Some("Captain Credible"),
      ~album=Some("Fantasy Mansion"),
      ~bpm=Some(180.),
      ~number=Some(1),
    ),
  );

let trackB: option(t) = None;

let tracks = [
  t(
    ~title="Fantasy Mansion",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Hot lips in the pool",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Organ master",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Crystal math",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Endless corridors",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Cloth",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Luxury estate",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="I want to go to fun town park",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Fantasy Mansion (LEIF remix)",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Forest moon (Harvey Steel non pan flute remake)",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title={j|Luxury estate (Lárus Sigurvin remix)|j},
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
  t(
    ~title="Main.h (Center of the Universe dub)",
    ~artist=Some("Captain Credible"),
    ~album=Some("Fantasy Mansion"),
    ~bpm=Some(180.),
    ~number=Some(1),
  ),
];
