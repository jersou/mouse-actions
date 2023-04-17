import { useState } from "react";
import { ShapeCreator } from "./ShapeCreator";
import { ShapeSvg } from "./ShapeSvg";
import { IconButton } from "@mui/material";
import DeleteIcon from "@mui/icons-material/Delete";
import EditIcon from "@mui/icons-material/Edit";
import AddIcon from "@mui/icons-material/Add";

export function ShapeEditor({
  coords,
  enableDelete,
  setShape,
  deleteShape,
  addShape,
}: {
  coords: number[];
  enableDelete: boolean;
  setShape: (shape: number[]) => unknown;
  deleteShape: () => unknown;
  addShape: () => unknown;
}) {
  const [shapeCreatorIsEnable, setShapeCreatorIsEnable] = useState(false);
  return (
    <div style={{ display: "flex" }}>
      <ShapeCreator
        enable={shapeCreatorIsEnable}
        setShape={(shape) => {
          setShapeCreatorIsEnable(false);
          setShape(shape);
        }}
      />
      <ShapeSvg coords={coords} />
      <div style={{ display: "flex", flexDirection: "column" }}>
        {enableDelete && (
          <IconButton
            title="Delete the shape"
            color="warning"
            onClick={deleteShape}
          >
            <DeleteIcon />
          </IconButton>
        )}
        <IconButton
          title="Re-draw the shape"
          color="primary"
          onClick={() => setShapeCreatorIsEnable(true)}
        >
          <EditIcon />
        </IconButton>
        <IconButton title="Add a new shape" onClick={addShape} color="primary">
          <AddIcon />
        </IconButton>
      </div>
    </div>
  );
}
