import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import "@fontsource/open-sans/400.css";
import "@fontsource/open-sans/700.css";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
