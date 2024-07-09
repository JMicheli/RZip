import React from "react";
import ReactDOM from "react-dom/client";
import RzipApp from "./layout/RzipApp";
import "./styles.css";
import RzipConfigProvider from "./RzipConfigProvider";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RzipConfigProvider>
      <RzipApp />
    </RzipConfigProvider>
  </React.StrictMode>
);
