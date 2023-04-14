import { ShapeSvg } from "./components/ShapeSvg.tsx";
import { Binding } from "./components/Binding.tsx";
import { useEffect, useRef, useState } from "react";
import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";

export type Point = { x: number; y: number };

export function useCoords(listenerEnable: boolean) {
  const [coords, setCoords] = useState<number[]>([]);
  const pointsHistory = useRef<Point[]>([]);
  const mouseState = useRef(false);

  const mousemove = (event) => {
    if (mouseState.current) {
      pointsHistory.current.push({ x: event.x, y: event.y });
    }
  };
  const mousedown = (event) => {
    mouseState.current = true;
    pointsHistory.current = [];
  };
  const mouseup = (event) => {
    mouseState.current = false;
    if (pointsHistory.current.length > 10) {
      const raw: { x: number; y: number }[] = pointsHistory.current;
      const minX = Math.min(...raw.map((point) => point.x));
      const minY = Math.min(...raw.map((point) => point.y));
      const maxX = Math.max(...raw.map((point) => point.x));
      const maxY = Math.max(...raw.map((point) => point.y));
      const width = maxX - minX;
      const height = maxY - minY;
      const size = Math.max(width, height);
      const normalizedCoords: number[] = [];
      for (const { x, y } of raw) {
        normalizedCoords.push(Math.round(((x - minX) * 1000) / size));
        normalizedCoords.push(Math.round(((y - minY) * 1000) / size));
      }
      setCoords(normalizedCoords);
    }
  };

  useEffect(() => {
    if (listenerEnable) {
      window.addEventListener("mousemove", mousemove);
      window.addEventListener("mousedown", mousedown);
      window.addEventListener("mouseup", mouseup);
      return () => {
        window.removeEventListener("mousemove", mousemove);
        window.removeEventListener("mousedown", mousedown);
        window.removeEventListener("mouseup", mouseup);
      };
    }
  }, [listenerEnable]);

  return coords;
}

export function ShapeCreator() {
  const coords = useCoords(true);
  console.log({ coords });

  return (
    <div
      style={{
        position: "absolute",
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
        backgroundColor: "#ccc",
      }}
    >
      <div>Draw a new shape with left button !</div>
    </div>
  );
}

// TODO add sides/corners
export default function Config() {
  const [config, setConfig] = useState(undefined);
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
    const newVconfig = await invoke("get_json_config");
    setConfig(JSON.parse(newVconfig));
  };
  useEffect(() => {
    refreshConfig();
  }, []);

  return (
    <div class="gap-2 w-full">
      {config && (
        <div>
          <div>shape_button: {config.shape_button}</div>
          {config.bindings.map((binding) => (
            <Binding binding={binding} />
          ))}
        </div>
      )}
      {coords && coords.length > 0 ? (
        <ShapeSvg coords={coords} />
      ) : (
        <ShapeCreator />
      )}

      <Button onClick={() => setCoords([])}>reset</Button>
      <Button onClick={() => refreshConfig()}>refreshConfig</Button>
    </div>
  );
}
