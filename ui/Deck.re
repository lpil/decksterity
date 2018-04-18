let component = ReasonReact.statelessComponent("Deck");

let deckHeight = 80;

let deckHeightPx = deckHeight |> Css.px;

let deckStyle =
  Theme.(
    style([
      displayGrid,
      gridTemplateColumns("300px 1fr"),
      height(deckHeightPx),
      paddingBottom(gapSize),
      lastChild([paddingBottom(zero)]),
    ])
  );

let waveformStyle =
  Theme.(
    style([
      backgroundColor(pink),
      /* padding(gapSize), */
      /* gridTemplateColumns("300px 1fr"), */
    ])
  );

let infoStyle = Theme.(style([]));

let infoElem =
  fun
  | None => <div className=infoStyle />
  | Some((track: Track.t)) =>
    <div className=infoStyle>
      <div className="">
        (track |> Track.title |> ReasonReact.stringToElement)
      </div>
      <div className="">
        (track |> Track.artist |> ReasonReact.stringToElement)
      </div>
    </div>;

let make = (~track, _children) => {
  ...component,
  render: _self =>
    <div className=deckStyle>
      (infoElem(track))
      <div className=waveformStyle />
    </div>,
};
