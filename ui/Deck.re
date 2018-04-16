type id =
  | A
  | B;

let idToString =
  fun
  | A => "A"
  | B => "B";

let component = ReasonReact.statelessComponent("Deck");

module Styles = {
  open Theme;
  let deck = style([display(grid), padding(gapSize)]);
  let info = style([]);
  let albumArtSize = px(56);
  let albumArt = style([height(albumArtSize), width(albumArtSize)]);
};

let make = (~id, _children) => {
  ...component,
  render: _self => {
    let _ = 1;
    <div className=Styles.deck>
      <div className=Styles.info>
        (ReasonReact.stringToElement("Info"))
      </div>
      (ReasonReact.stringToElement("Deck " ++ idToString(id)))
    </div>;
  },
};
