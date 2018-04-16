let component = ReasonReact.statelessComponent("Page");

let pageStyle = Theme.(style([padding(gapSize)]));

let make = (~tracks, _children) => {
  ...component,
  render: _self => {
    let _ = tracks;
    <div className=pageStyle>
      <Deck track=Track.trackA />
      <Deck track=Track.trackB />
      (ReasonReact.stringToElement("hi thar"))
      (ReasonReact.stringToElement("hi thar"))
    </div>;
  },
};
