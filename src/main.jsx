import "bulma/css/bulma.css";
import { createRoot } from "react-dom/client";
import wasm from "../wasm/pkg";
import App from "./App";

await wasm();
createRoot(document.querySelector("#content")).render(<App />);
