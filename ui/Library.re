open Belt;

let component = ReasonReact.statelessComponent("Library");

let trackListStyle =
  Theme.(style([margin(zero), padding(gapSize), paddingTop(zero)]));

let trackStyle =
  Theme.(
    style([
      displayGrid,
      gridTemplateColumns("auto 6em"),
      padding2(~h=zero, ~v=gapSize),
    ])
  );

let artistInfoStyle =
  Theme.(style([color(subtleWhite)]));

let numberStyle = Theme.(style([textAlign(`right)]));

let bpmEl = track =>
  track
  |> Track.bpm
  |> Option.map(_, string_of_float)
  |> Option.getWithDefault(_, "???")
  |> (n => n ++ " BPM")
  |> ReasonReact.stringToElement;

let titleEl = track => track |> Track.title |> ReasonReact.stringToElement;

let artistEl = track => track |> Track.artist |> ReasonReact.stringToElement;

let albumEl = track =>
  track
  |> Track.album
  |> Option.getWithDefault(_, "")
  |> ReasonReact.stringToElement;

let trackLi = track =>
  <li className=trackStyle key=(Track.id(track))>
    <div>
      <span> (titleEl(track)) </span>
      <div className=artistInfoStyle>
        <span> (artistEl(track)) </span>
        (ReasonReact.stringToElement(" - "))
        <span> (albumEl(track)) </span>
      </div>
    </div>
    <div className=numberStyle> (bpmEl(track)) </div>
  </li>;

let libraryStyle = Theme.(style([]));

let make = (~tracks: list(Track.t), _children) => {
  ...component,
  render: _self =>
    <div className=libraryStyle>
      <ul className=trackListStyle>
        (
          tracks
          |> List.map(_, trackLi)
          |> List.toArray
          |> ReasonReact.arrayToElement
        )
      </ul>
    </div>,
};
