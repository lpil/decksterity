open Belt;

let component = ReasonReact.statelessComponent("Library");

let libraryStyle = Theme.(style([]));

let trackLi = track =>
  <div key=Track.id(track)> (track |> Track.title |> ReasonReact.stringToElement) </div>;

let make = (~tracks: list(Track.t), _children) => {
  ...component,
  render: _self =>
    <div className=libraryStyle>
      (ReasonReact.stringToElement("Library"))
      <ul>
        (
          tracks
          |> List.map(_, trackLi)
          |> List.toArray
          |> ReasonReact.arrayToElement
        )
      </ul>
    </div>,
};
