import { ScreenEdges } from "./ScreenEdges";
import { BindingType } from "./config.type";
import { ModifiersSelector } from "./ModifiersSelector";
import { EventTypeSelector } from "./EventTypeSelector";
import { ButtonSelector } from "./ButtonSelector";
import { IconButton, TextField } from "@mui/material";
import { memo } from "react";
import { ShapeEditor } from "./ShapeEditor";
import AddIcon from "@mui/icons-material/Add";
import DeleteIcon from "@mui/icons-material/Delete";
import { isEqual } from "lodash";

export function Binding({
  binding,
  setBinding,
  addBinding,
  deleteBinding,
}: {
  binding: BindingType;
  setBinding?: (binding: BindingType) => unknown;
  addBinding: (binding: BindingType) => unknown;
  deleteBinding: (binding: BindingType) => unknown;
}) {
  return (
    <div
      style={{
        marginBottom: 20,
        padding: 10,
        maxWidth: 1000,
        display: "grid",
        gap: 10,
        // FIXME
        gridTemplateColumns: "30px 90px 1fr 170px",
        borderBottom: "solid #aaa 2px",
      }}
    >
      <div style={{ display: "flex", flexDirection: "column" }}>
        <IconButton
          title="Delete the binding"
          color="warning"
          onClick={() => deleteBinding(binding)}
        >
          <DeleteIcon />
        </IconButton>
        <IconButton
          title="Add a binding"
          color="primary"
          onClick={() => addBinding(binding)}
        >
          <AddIcon />
        </IconButton>
      </div>
      <EventTypeSelector
        eventType={binding.event.event_type}
        setEventType={(evType) => {
          const newBinding = structuredClone({
            ...binding,
            event: { ...binding.event, event_type: evType },
          });
          if (
            evType === "Shape" &&
            (!binding.event.shapes_xy || binding.event.shapes_xy.length === 0)
          ) {
            newBinding.event.shapes_xy = [[]];
          }
          // if (evType === "Shape") {
          //   newBinding.event.edges = [];
          // } else {
          //   newBinding.event.shapes_xy = [];
          // }
          setBinding?.(newBinding);
        }}
      />
      <div style={{ flexDirection: "column", flex: 1, display: "flex" }}>
        <div style={{ display: "flex" }}>
          <div style={{ marginRight: 10 }}>
            <ButtonSelector
              button={binding.event.button}
              setButton={(button) =>
                setBinding?.(
                  structuredClone({
                    ...binding,
                    event: { ...binding.event, button },
                  })
                )
              }
            />
          </div>

          <TextField
            size="small"
            style={{ flex: 1 }}
            label="Comment"
            variant="outlined"
            value={binding.comment}
            onChange={(e) => {
              setBinding?.(
                structuredClone({
                  ...binding,
                  comment: e.target.value,
                })
              );
            }}
          />
        </div>
        <div style={{ display: "flex", flex: 0 }}>
          <TextField
            size="small"
            style={{ flex: 1, marginTop: 10, marginBottom: 10 }}
            label="Command"
            variant="outlined"
            value={binding.cmd_str}
            onChange={(e) => {
              setBinding?.(
                structuredClone({
                  ...binding,
                  cmd_str: e.target.value,
                })
              );
            }}
          />
        </div>
        <ModifiersSelector
          modifiers={binding.event.modifiers || []}
          setModifiers={(modifiers) =>
            setBinding?.(
              structuredClone({
                ...binding,
                event: { ...binding.event, modifiers },
              })
            )
          }
        />
      </div>
      <div>
        {binding.event.event_type === "Shape" ? (
          <div>
            {binding.event.shapes_xy?.map((coords, i) => (
              <ShapeEditor
                key={i}
                enableDelete={(binding.event.shapes_xy?.length || 0) > 1}
                coords={coords}
                setShape={(shape) => {
                  const shapes_xy = [...(binding.event.shapes_xy || [])];
                  shapes_xy[i] = shape;
                  setBinding?.(
                    structuredClone({
                      ...binding,
                      event: { ...binding.event, shapes_xy },
                    })
                  );
                }}
                addShape={() => {
                  const shapes_xy = [...(binding.event.shapes_xy || [])];
                  shapes_xy.splice(i + 1, 0, []);
                  setBinding?.(
                    structuredClone({
                      ...binding,
                      event: { ...binding.event, shapes_xy },
                    })
                  );
                }}
                deleteShape={() => {
                  const shapes_xy = [...(binding.event.shapes_xy || [])];
                  shapes_xy.splice(i, 1);
                  setBinding?.(
                    structuredClone({
                      ...binding,
                      event: { ...binding.event, shapes_xy },
                    })
                  );
                }}
              />
            ))}
          </div>
        ) : (
          <div>
            <ScreenEdges
              edges={binding.event.edges ?? []}
              setEdges={(edges) =>
                setBinding?.(
                  structuredClone({
                    ...binding,
                    event: { ...binding.event, edges },
                  })
                )
              }
            />
          </div>
        )}
      </div>
    </div>
  );
}

export const BindingMemo = memo(Binding, (prev, next) =>
  isEqual(prev.binding, next.binding)
);
