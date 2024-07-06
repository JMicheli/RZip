import React from "react";
import ReactDOM from "react-dom/client";
import UiRoot from "./UiRoot";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <UiRoot />
  </React.StrictMode>
);
