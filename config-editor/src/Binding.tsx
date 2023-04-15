import { ShapeSvg } from "./ShapeSvg";
import { ScreenEdges } from "./ScreenEdges";
import { BindingType } from "./config.type";
import { ModifiersSelector } from "./ModifiersSelector";

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
        textAlign: "left",
        border: "solid black 2px",
        borderRadius: 15,
        marginBottom: 20,
        padding: 10,
        maxWidth: 700,
        display: "flex",
      }}
    >
      <div style={{ flex: 1 }}>
        <div>{binding.comment}</div>
        <div>
          {binding.event.event_type} with {binding.event.button} button
        </div>
        <div>
          trigger the command <br />
          {JSON.stringify(binding.cmd)}
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
        <div className="px-5">
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
        {binding.event.shapes_xy && (
          <div className="flex">
            {binding.event.shapes_xy?.map((coords) => (
              <ShapeSvg coords={coords} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
