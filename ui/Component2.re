type state = {
  count: int,
  show: bool,
};

type action =
  | Increment
  | Decrement
  | Toggle;

let component = ReasonReact.reducerComponent("Example");

let reducer = (action, state) =>
  switch (action) {
  | Increment => ReasonReact.Update({...state, count: state.count + 1})
  | Decrement => ReasonReact.Update({...state, count: state.count - 1})
  | Toggle => ReasonReact.Update({...state, show: ! state.show})
  };

let initialState = () => {count: 0, show: true};

let make = (~greeting, _children) => {
  ...component,
  initialState,
  reducer,
  render: self => {
    let message = "The number is " ++ string_of_int(self.state.count) ++ "!";
    <div>
      <button onClick=(_event => self.send(Decrement))>
        (ReasonReact.stringToElement("Decrement"))
      </button>
      <button onClick=(_event => self.send(Increment))>
        (ReasonReact.stringToElement("Increment"))
      </button>
      <div> (ReasonReact.stringToElement(message)) </div>
      <button onClick=(_event => self.send(Toggle))>
        (ReasonReact.stringToElement("Toggle greeting"))
      </button>
      (
        self.state.show ?
          ReasonReact.stringToElement(greeting) : ReasonReact.nullElement
      )
    </div>;
  },
};
