include Css;

/*
  Colors
 */
let darkGrey = rgb(24, 24, 24);

let white = rgb(255, 255, 255);

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
global("body", [margin(zero)]);

global(
  "*",
  [boxSizing(borderBox), color(fontColor), fontFamily("sans-serif")],
);

global("ul", [paddingLeft(zero)]);

/*
  Css extensions
 */
let displayGrid = unsafe("display", "grid");

let gridTemplateColumns = x => unsafe("gridTemplateColumns", x);
