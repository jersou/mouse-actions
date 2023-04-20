import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { BindingMemo } from "./Binding";
import { BindingType, ButtonType, ConfigType } from "./config.type";
import { ButtonSelector } from "./ButtonSelector";
import { Button, ButtonGroup, Typography } from "@mui/material";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import StopIcon from "@mui/icons-material/Stop";
import SaveIcon from "@mui/icons-material/Save";
import UndoIcon from "@mui/icons-material/Undo";
import GestureIcon from "@mui/icons-material/Gesture";
import AddIcon from "@mui/icons-material/Add";
import { AppSkeleton } from "./AppSkeleton";

export default function App() {
  const [isLoading, setIsLoading] = useState(false);
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
    setIsLoading(true);
    setTimeout(async () => {
      const newVconfig: ConfigType = await invoke("get_config");
      newVconfig.bindings.forEach((b) => (b.uid = self.crypto.randomUUID()));
      setConfig(newVconfig);
      setIsLoading(false);
    }, 100);
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
    (binding: BindingType) => {
      setConfig((prevConfig) => {
        const index = prevConfig?.bindings.findIndex(
          (b) => b.uid === binding.uid
        );
        const newConfig: ConfigType = {
          shape_button: prevConfig?.shape_button || "Right",
          bindings: [...(prevConfig?.bindings || [])],
        };
        if (index !== undefined) {
          newConfig.bindings.splice(index, 1);
        }
        return newConfig;
      });
    },
    [setConfig]
  );

  const addBinding = useCallback(
    (binding?: BindingType) => {
      setConfig((prevConfig) => {
        const index = prevConfig?.bindings.findIndex(
          (b) => b.uid === binding?.uid
        );

        const newConfig: ConfigType = {
          shape_button: prevConfig?.shape_button || "Right",
          bindings: [...(prevConfig?.bindings || [])],
        };
        newConfig.bindings?.splice((index ?? -1) + 1, 0, {
          uid: self.crypto.randomUUID(),
          cmd: ["TODO"],
          comment: "TODO",
          event: {
            button: "Right",
            event_type: "Click",
            edges: [],
            modifiers: ["ControlLeft"],
            shapes_xy: [],
          },
        });
        return newConfig;
      });
    },
    [setConfig]
  );

  const setShapeButton = (shape_button: ButtonType) => {
    setConfig((prevConfig) => ({
      bindings: [...(prevConfig?.bindings || [])],
      shape_button,
    }));
  };

  const saveConfig = async () => {
    await invoke("save_config", { newConfig: config });
  };

  return config && !isLoading ? (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        position: "absolute",
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
      }}
    >
      <div
        style={{
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
          <ButtonSelector
            button={config.shape_button}
            setButton={setShapeButton}
          />
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
          <Button color="warning" variant="contained" onClick={refreshConfig}>
            <UndoIcon /> Reload config
          </Button>
          <Button variant="contained" onClick={saveConfig}>
            <SaveIcon /> Save
          </Button>
        </ButtonGroup>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          overflow: "auto",
        }}
      >
        {config.bindings.map((binding, index) => (
          <BindingMemo
            key={binding.uid || index}
            binding={binding}
            setBinding={onNewBinding}
            addBinding={addBinding}
            deleteBinding={deleteBinding}
          />
        ))}
        <div
          style={{
            width: "100%",
            paddingBottom: 8,
            marginBottom: 8,
            display: "flex",
            justifyContent: "center",
          }}
        >
          <Button
            variant="contained"
            size="small"
            onClick={() =>
              addBinding(config?.bindings[config.bindings.length - 1])
            }
          >
            <AddIcon /> Add a binding
          </Button>
        </div>
      </div>
    </div>
  ) : (
    <AppSkeleton />
  );
}
