open Css;

/*
  bs-css extensions, adding support for CSS features it does
  not yet cover.
 */
let rec string_of_length =
  fun
  | `calc(`add, a, b) =>
    "calc(" ++ string_of_length(a) ++ " + " ++ string_of_length(b) ++ ")"
  | `calc(`sub, a, b) =>
    "calc(" ++ string_of_length(a) ++ " - " ++ string_of_length(b) ++ ")"
  | `ch(x) => string_of_float(x) ++ "ch"
  | `cm(x) => string_of_float(x) ++ "cm"
  | `em(x) => string_of_float(x) ++ "em"
  | `ex(x) => string_of_float(x) ++ "ex"
  | `mm(x) => string_of_float(x) ++ "mm"
  | `percent(x) => string_of_float(x) ++ "%"
  | `pt(x) => string_of_int(x) ++ "pt"
  | `px(x) => string_of_int(x) ++ "px"
  | `rem(x) => string_of_float(x) ++ "rem"
  | `vh(x) => string_of_float(x) ++ "vh"
  | `vmax(x) => string_of_float(x) ++ "vmax"
  | `vmin(x) => string_of_float(x) ++ "vmin"
  | `vw(x) => string_of_float(x) ++ "vw"
  | `zero => "0";

let gridTemplateColumns = (sizes: list(length)) =>
  sizes
  |> List.map(string_of_length)
  |> String.concat(" ")
  |> unsafe("grid-template-columns");

let grid = `grid;

let display = x =>
  unsafe(
    "display",
    switch (x) {
    | `block => "block"
    | `inline => "inline"
    | `inlineBlock => "inline-block"
    | `none => "none"
    | `flex => "flex"
    | `inlineFlex => "inline-flex"
    | `grid => "grid"
    },
  );
