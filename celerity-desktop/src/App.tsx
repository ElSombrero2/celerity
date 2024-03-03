import { useEffect } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api";

function App() {

  useEffect(() => {
    invoke('get_configuration')
    .then((res) => console.log(res))
  }, []);

  return (
    <div className="container">
      <h1>Hello World</h1>
    </div>
  );
}

export default App;
