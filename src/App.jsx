import { tsne } from "../wasm/pkg";

export default function App() {
  return (
    <div>
      <h1>hello</h1>
      <button
        onClick={() => {
          console.log(tsne());
        }}
      >
        click
      </button>
    </div>
  );
}
