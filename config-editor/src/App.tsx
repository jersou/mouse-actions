import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Binding } from "./Binding";
import { ConfigType } from "./config.type";
import { useCoords } from "./UseCoords";
import { ButtonSelector } from "./ButtonSelector";
import {Button, CircularProgress} from "@mui/material";


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

  return config ? (
    <div>
      <div
        style={{
          position: "sticky",
          top: 0,
          backgroundColor: "#fff",
          display: "flex",
          flexDirection: "row",
          borderBottom: "solid #888 1px",
          padding: 10,
          zIndex: 10,
          boxShadow: "0 2px 5px rgb(152, 151, 151)",
          justifyContent:"space-between",
          marginBottom:10
        }}
      >
        <div>
          Shape button : <ButtonSelector button={config.shape_button} />
        </div>
        <div>
        <Button variant="contained"  onClick={()=>invoke("stop")}>Stop</Button>
        <Button variant="contained" onClick={()=>invoke("start")}>Start</Button>
        <Button variant="contained">Save (todo)</Button>
        </div>
      </div>
      <div>
        {config.bindings.map((binding) => (
          <Binding binding={binding} />
        ))}
      </div>
    </div>
  ) : (
    <div
      style={{
        position: "absolute",
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <CircularProgress size={100} />
    </div>
  );
}
