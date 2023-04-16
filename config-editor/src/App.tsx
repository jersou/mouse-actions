import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { BindingMemo } from "./Binding";
import { BindingType, ConfigType } from "./config.type";
import { ButtonSelector } from "./ButtonSelector";
import {
  Button,
  ButtonGroup,
  CircularProgress,
  Typography,
} from "@mui/material";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import StopIcon from "@mui/icons-material/Stop";
import SaveIcon from "@mui/icons-material/Save";
import GestureIcon from "@mui/icons-material/Gesture";

export default function App() {
  const [defaultConfigPath, setGreetMsg] = useState("");
  const [version, setVersion] = useState("");
  const [config, setConfig] = useState<ConfigType>();
  const [shapeRecording, setShapeRecording] = useState(false);

  async function getDefaultConfigPath() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("get_default_config_path"));
  }

  useEffect(() => {
    invoke("get_version").then((v: any) => setVersion(v));
  }, []);

  // const [coords, setCoords] = useState<number[]>([
  //   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000, 1000,
  // ]);
  // const newCoords = useCoords(shapeRecording);
  // useEffect(() => {
  //   if (newCoords?.length) {
  //     setCoords(newCoords);
  //   }
  // }, [setCoords, newCoords]);

  const refreshConfig = async () => {
    const newVconfig: ConfigType = await invoke("get_config");
    newVconfig.bindings.forEach((b) => (b.uid = self.crypto.randomUUID()));
    setConfig(newVconfig);
  };
  useEffect(() => {
    refreshConfig();
  }, []);

  const onNewBinding = useCallback(
    (newBinding: BindingType) => {
      setConfig((prevConfig) => {
        if (prevConfig) {
          const newConfig = {
            ...prevConfig,
            binding: [...prevConfig?.bindings],
          };
          const index = prevConfig?.bindings.findIndex(
            (b) => b.uid === newBinding.uid
          );
          if (index >= 0) {
            newConfig.bindings[index] = newBinding;
            setConfig(newConfig);
          }
          return newConfig;
        } else {
          return prevConfig;
        }
      });
    },
    [setConfig]
  );

  const deleteBinding = useCallback(
    (index: number) => {
      setConfig((prevConfig) => {
        const newConfig: ConfigType = {
          shape_button: prevConfig?.shape_button || "Right",
          bindings: [...(prevConfig?.bindings || [])],
        };
        newConfig.bindings.splice(index, 1);
        return newConfig;
      });
    },
    [setConfig]
  );

  const addBinding = useCallback(
    (index: number) => {
      setConfig((prevConfig) => {
        const newConfig: ConfigType = {
          shape_button: prevConfig?.shape_button || "Right",
          bindings: [...(prevConfig?.bindings || [])],
        };
        newConfig.bindings?.splice(index, 0, {
          uid: self.crypto.randomUUID(),
          cmd: ["TODO"],
          comment: "TODO",
          event: {
            button: "Right",
            event_type: "Click",
            edges: ["Top"],
            modifiers: [],
            shapes_xy: [],
          },
        });
        return newConfig;
      });
    },
    [setConfig]
  );

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
          justifyContent: "space-between",
          marginBottom: 10,
        }}
      >
        <div style={{ display: "flex", alignItems: "center" }}>
          <GestureIcon />
          <Typography style={{ marginLeft: 10, marginRight: 10 }}>
            Shape button :
          </Typography>
          <ButtonSelector button={config.shape_button} />
        </div>
        <ButtonGroup>
          <Button
            color="warning"
            variant="contained"
            onClick={() => invoke("stop")}
          >
            <StopIcon /> Stop
          </Button>
          <Button
            variant="contained"
            onClick={() => invoke("start")}
            color="success"
          >
            <PlayArrowIcon /> Start
          </Button>
          <Button variant="contained">
            <SaveIcon /> Save (todo)
          </Button>
        </ButtonGroup>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        {config.bindings.map((binding, index) => (
          <BindingMemo
            key={index}
            binding={binding}
            setBinding={onNewBinding}
            addBinding={() => addBinding(index + 1)}
            deleteBinding={() => deleteBinding(index)}
          />
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
