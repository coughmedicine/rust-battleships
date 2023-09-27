import { AddingState, Command, ShipDir, ws } from "./main";
import "./AddingGrid.css";
import { useState } from "react";

function AddingGrid(props: { state: AddingState }) {
  const [dir, setDir] = useState<ShipDir>("Horz");

  const state = props.state;

  const hasShips: boolean[][] = [];
  for (let i = 0; i < state.size; i++) {
    const temp = [];
    for (let j = 0; j < state.size; j++) {
      temp.push(false);
    }
    hasShips.push(temp);
  }
  for (const s of state.ships) {
    for (const l of s) {
      hasShips[l.y][l.x] = true;
    }
  }

  const onDirChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setDir(e.target.value as ShipDir);
  };

  return (
    <>
      {[...Array(state.size).keys()].map((y) => (
        <div key={y}>
          {[...Array(state.size).keys()].map((x) => (
            <button
              className="grid-field"
              key={x}
              onClick={() => {
                const comm: Command = { type: "AddShip", loc: { x, y }, dir };
                ws.send(JSON.stringify(comm));
                console.log(`You clicked ${x}, ${y}`);
              }}
            >
              {hasShips[y][x] ? `🚢` : `🌊`}
            </button>
          ))}
        </div>
      ))}
      <div>
        <input
          type="radio"
          name="dir"
          value="Horz"
          id="horz-radio"
          checked={dir === "Horz"}
          onChange={onDirChange}
        />
        <label htmlFor="horz-radio">Horizontal</label>
        <input
          type="radio"
          name="dir"
          value="Vert"
          id="vert-radio"
          checked={dir === "Vert"}
          onChange={onDirChange}
        />
        <label htmlFor="vert-radio">Vertical</label>
      </div>
    </>
  );
}

export default AddingGrid;
