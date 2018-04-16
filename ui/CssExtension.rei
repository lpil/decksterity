let gridTemplateColumns: list(Css.length) => Css.rule;

let grid: [> | `grid];

let display:
  [ | `grid | `flex | `block | `inline | `inlineBlock | `none | `inlineFlex ] =>
  Css.rule;

