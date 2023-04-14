import { ShapeSvg } from "./ShapeSvg";
import { ScreenEdges } from "./ScreenEdges";

export function Binding({ binding }) {
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
        {binding.event.modifiers && (
          <div className="px-5">
            modifiers: {JSON.stringify(binding.event.modifiers)}
          </div>
        )}
      </div>
      <div>
        <div className="px-5">
          <ScreenEdges edges={binding.event.edges ?? []} />
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
