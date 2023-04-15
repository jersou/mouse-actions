import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Binding } from "./Binding";
import { ConfigType } from "./config.type";
import { useCoords } from "./UseCoords";
import {ButtonSelector} from "./ButtonSelector";

export default function App() {
  const [defaultConfigPath, setGreetMsg] = useState("");
  const [version, setVersion] = useState("");

  async function getDefaultConfigPath() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("get_default_config_path"));
  }

  useEffect(() => {
    invoke("get_version").then((v: any) => setVersion(v));
  }, []);

  const [config, setConfig] = useState<ConfigType>();
  const [coords, setCoords] = useState<number[]>([
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000, 1000,
  ]);
  const newCoords = useCoords(!(coords && coords.length > 0));
  useEffect(() => {
    if (newCoords?.length) {
      setCoords(newCoords);
    }
  }, [setCoords, newCoords]);

  const refreshConfig = async () => {
    const newVconfig: ConfigType = await invoke("get_config");
    setConfig(newVconfig);
  };
  useEffect(() => {
    refreshConfig();
  }, []);

  return (
    <div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "left", borderBottom: "solid #000 1px"
        }}
      >
        <img
          src="/logo.svg"
          width={16}
          alt="Mouse Actions logo"
          style={{ marginRight: 10 }}
        />
        {version}

        {config && (
          <div>
            Shape button : <ButtonSelector button={config.shape_button}/>
          </div>
        )}
      </div>
      <div>
        {config &&
          config.bindings.map((binding) => <Binding binding={binding} />)}
      </div>
    </div>
  );
}
