import AddingGrid from "./AddingGrid";
import "./App.css";
import { useGameState } from "./main";

function App() {
  const state = useGameState();

  return (
    <>
      {state.type === "Waiting" && <p>Waiting for second player...</p>}
      {state.type === "Adding" && <AddingGrid state={state} />}
      <p id="state">{JSON.stringify(state)}</p>
    </>
  );
}

export default App;
