import React from "react";
import ReactDOM from "react-dom/client";
import UIRoot from "./UIRoot";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <UIRoot />
  </React.StrictMode>
);
