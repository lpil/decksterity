let component = ReasonReact.statelessComponent("Page");

let pageStyle = Theme.(style([padding(gapSize)]));

let make = (~tracks, _children) => {
  ...component,
  render: _self =>
    <div className=pageStyle>
      <Deck track=Track.trackA />
      <Deck track=Track.trackB />
      <Library tracks />
    </div>,
};
