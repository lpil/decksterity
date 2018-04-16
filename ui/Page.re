let component = ReasonReact.statelessComponent("Page");

let make = (~tracks, _children) => {
  ...component,
  render: _self =>
    <div >
      <Deck id=Deck.A />
      <Deck id=Deck.B />
      (ReasonReact.stringToElement("hi thar"))
      (ReasonReact.stringToElement("hi thar"))
    </div>,
};
