import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { create } from "zustand";

export type Location = {
  x: number;
  y: number;
};

export type Player = "Player1" | "Player2";

export type AddingState = {
  type: "Adding";
  ships: [[Location]];
  size: number;
};
export type GuessingState = {
  type: "Guessing";
};
export type WonState = {
  type: "Won";
  who: Player;
};

export type GameState =
  | { type: "Waiting" }
  | AddingState
  | GuessingState
  | WonState;

export type AddShipCommand = {
  type: "AddShip";
  loc: Location;
  dir: ShipDir;
};
export type ShipDir = "Horz" | "Vert";
export type GuessPosCommand = {
  type: "GuessPos";
  loc: Location;
};

export type Command = AddShipCommand | GuessPosCommand;

export const useGameState = create<GameState>(() => ({
  type: "Waiting",
}));
export const ws = new WebSocket("ws://127.0.0.1:3000/ws");
ws.onmessage = (event) => {
  if (typeof event.data === "string") {
    const state = JSON.parse(event.data);
    useGameState.setState(state as GameState, true);
  }
};
ws.onclose = () => {
  console.log("connection closed");
};

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
