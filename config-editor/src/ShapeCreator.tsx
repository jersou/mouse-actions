import { useCoords } from "./UseCoords";
import { useEffect } from "react";

export function ShapeCreator({
  enable,
  setShape,
}: {
  enable: boolean;
  setShape: (shape: number[]) => unknown;
}) {
  const coords = useCoords(enable);
  useEffect(() => {
    if (coords.length) {
      setShape(coords);
    }
  }, [coords]);

  return enable ? (
    <div
      style={{
        position: "absolute",
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
        backgroundColor: "#ccc",
        zIndex: 10,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        fontSize: 40,
        userSelect: "none",
      }}
    >
      <div>Draw a new shape with left button here !</div>
    </div>
  ) : null;
}
