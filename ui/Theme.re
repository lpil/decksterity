include Css;

/*
  Colors
 */
let darkGrey = rgb(24, 24, 24);

let white = rgb(255, 255, 255);

let subtleWhite = rgba(255, 255, 255, 0.6);

let black = rgb(0, 0, 0);

let pink = deeppink;

/*
  Color assignments
 */
let bgColor = darkGrey;

let fontColor = white;

/*
  Sizes
 */
let gapSize = px(16);

/*
  Global styles
 */
global(
  "body",
  [margin(zero), backgroundColor(bgColor), color(fontColor)],
);

global(
  "*",
  [
    boxSizing(borderBox),
    fontFamily("sans-serif"),
    fontSize(16 |> px),
    lineHeight(1.3),
  ],
);

global("ul", [paddingLeft(zero)]);

global("li", [listStyleType(`none)]);

/*
  Css extensions
 */
let displayGrid = unsafe("display", "grid");

let gridTemplateColumns = x => unsafe("gridTemplateColumns", x);
