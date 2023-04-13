import { useEffect, useRef, useState } from "preact/hooks";
import { Button } from "../components/Button.tsx";
import { ShapeSvg } from "../components/ShapeSvg.tsx";
import { Binding } from "../components/Binding.tsx";

export type Point = { x: number; y: number };

export function useCoords(listenerEnable: boolean) {
  const [coords, setCoords] = useState<Point[]>([]);
  const pointsHistory = useRef<Point[]>([]);
  const mouseState = useRef(false);

  const mousemove = (event) => {
    if (mouseState.target) {
      pointsHistory.target.push({ x: event.x, y: event.y });
    }
  };
  const mousedown = (event) => {
    mouseState.target = true;
    pointsHistory.target = [];
  };
  const mouseup = (event) => {
    mouseState.target = false;
    if (pointsHistory.target.length > 10) {
      const raw: { x: number; y: number }[] = pointsHistory.target;
      const minX = Math.min(...raw.map((point) => point.x));
      const minY = Math.min(...raw.map((point) => point.y));
      const maxX = Math.max(...raw.map((point) => point.x));
      const maxY = Math.max(...raw.map((point) => point.y));
      const width = maxX - minX;
      const height = maxY - minY;
      const size = Math.max(width, height);
      const normalizedCoords: number[] = [];
      for (const { x, y } of raw) {
        normalizedCoords.push(Math.round((x - minX) * 1000 / size));
        normalizedCoords.push(Math.round((y - minY) * 1000 / size));
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
  const [coords, setCoords] = useState<Point[]>([{ x: 0, y: 0 }]);
  const newCoords = useCoords(!(coords && coords.length > 0));
  useEffect(() => {
    if (newCoords?.length) {
      setCoords(newCoords);
    }
  }, [setCoords, newCoords]);

  const refreshConfig = async () => {
    const res = await fetch("http://localhost:8000/api/get_config");
    setConfig(await res.json());
  };
  useEffect(() => {
    refreshConfig();
  }, []);

  return (
    <div class="gap-2 w-full">
      {coords && coords.length > 0
        ? <ShapeSvg coords={coords} />
        : <ShapeCreator />}

      <Button onClick={() => setCoords([])}>reset</Button>
      <Button onClick={() => refreshConfig()}>refreshConfig</Button>
      {config && (
        <div>
          <div>shape_button: {config.shape_button}</div>
          {config.bindings.map((binding) => <Binding binding={binding} />)}
        </div>
      )}
      <pre>{JSON.stringify(config, null, "  ")}</pre>
    </div>
  );
}
