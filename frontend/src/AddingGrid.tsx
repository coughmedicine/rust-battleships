import { AddingState } from "./main";

function AddingGrid(props: { state: AddingState }) {
  const state = props.state;
  return [...Array(state.size).keys()].map((y) => (
    <div key={y}>
      {[...Array(state.size).keys()].map((x) => (
        <button
          key={x}
          onClick={() => {
            console.log(`You clicked ${x}, ${y}`);
          }}
        >{`${x}, ${y}`}</button>
      ))}
    </div>
  ));
}

export default AddingGrid;
