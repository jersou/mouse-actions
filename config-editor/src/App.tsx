import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import MouseActionRuntime from "./MouseActionRuntime";
import Config from "./Config";

function App() {
  const [defaultConfigPath, setGreetMsg] = useState("");
  async function getDefaultConfigPath() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("get_default_config_path"));
  }

  return (
    <div className="container">
      <h1>Mouse Action config editor</h1>
          <img src="/logo.svg" className="logo" alt="Mouse Actions logo" />

        <MouseActionRuntime />
        <Config />

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            getDefaultConfigPath();
          }}
        >
          <button type="submit">get default config path</button>
        </form>
      </div>
      <p>{defaultConfigPath}</p>
    </div>
  );
}

export default App;
