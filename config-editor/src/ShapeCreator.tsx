import {useCoords} from "./UseCoords";

export function ShapeCreator() {
  const coords = useCoords(true);
  console.log({coords});

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
