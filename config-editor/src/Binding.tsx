import { ShapeSvg } from "./ShapeSvg";
import { ScreenEdges } from "./ScreenEdges";
import { BindingType } from "./config.type";
import { ModifiersSelector } from "./ModifiersSelector";
import { EventTypeSelector } from "./EventTypeSelector";
import { ButtonSelector } from "./ButtonSelector";
import { TextField } from "@mui/material";

export function Binding({
  binding,
  setBinding,
}: {
  binding: BindingType;
  setBinding?: (binding: BindingType) => unknown;
}) {
  return (
    <div
      style={{
        marginBottom: 20,
        padding: 10,
        maxWidth: 800,
        display: "grid",
        gap: 10,
        gridTemplateColumns: "1fr 5fr 1fr",
        borderBottom: "solid #000 1px",
      }}
    >
      <EventTypeSelector
        eventType={binding.event.event_type}
        setEventType={(evType) =>
          setBinding?.(
            structuredClone({
              ...binding,
              event: { ...binding.event, event_type: evType },
            })
          )
        }
      />
      <div style={{ flex: 1 }}>
        <div style={{ display: "flex" }}>
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
          <TextField
            size="small"
            style={{ flex: 1 }}
            label="Comment"
            variant="outlined"
            value={binding.comment}
          />
        </div>

        <TextField
          size="small"
          style={{ flex: 1, marginTop: 10, marginBottom: 10 }}
          label="Command"
          variant="outlined"
          value={JSON.stringify(binding.cmd)}
        />
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
            {binding.event.shapes_xy?.map((coords) => (
              <ShapeSvg coords={coords} />
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
