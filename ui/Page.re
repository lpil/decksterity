let component = ReasonReact.statelessComponent("Page");

let headerStyle =
  Theme.(
    style([
      position(sticky),
      top(zero),
      padding(gapSize),
      backgroundColor(bgColor),
    ])
  );

let make = (~tracks, _children) => {
  ...component,
  render: _self =>
    <div>
      <div className=headerStyle>
        <Deck track=Track.trackA />
        <Deck track=Track.trackB />
      </div>
      <Library tracks />
    </div>,
};
