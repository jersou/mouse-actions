import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";
import MouseActionRuntime from "./MouseActionRuntime";
import Config from "./Config";

function App() {
  const [defaultConfigPath, setGreetMsg] = useState("");
  const [version, setVersion] = useState("");

  async function getDefaultConfigPath() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("get_default_config_path"));
  }

  useEffect(() => {
    invoke("get_version").then(v => setVersion(v))
  }, [])

  return (
    <div className="container">
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "center"
        }}>
        <div>
          <img src="/logo.svg" width={50}
               alt="Mouse Actions logo" style={{marginRight: 10}}/>
        </div>
        <h2>Mouse Action config editor {version}</h2>
      </div>

      <Config/>
      <MouseActionRuntime/>

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
