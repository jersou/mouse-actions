import {ShapeSvg} from "./ShapeSvg";
import {ScreenEdges} from "./ScreenEdges.tsx";
import {Button} from "./Button.tsx";

export function Binding({binding}) {

  const test = () => {
    binding.test = "OK"
  }

  return (
    <div
      className="shadow-lg"
      style={{
        border: "solid black 2px",
        borderRadius: 15,
        marginBottom: 20,
        padding: 10,
      }}
    >
      <div className="font-medium">{binding.comment}</div>
      <div className="px-5">
        {binding.event.event_type} with {binding.event.button}{" "}
        button trigger the command
        {JSON.stringify(binding.cmd)}
      </div>
      {binding.event.modifiers && (
        <div className="px-5">
          modifiers: {JSON.stringify(binding.event.modifiers)}
        </div>
      )}
      <div className="px-5">
        <ScreenEdges edges={binding.event.edges ?? []}/>
      </div>
      {binding.event.shapes_xy && (
        <div className="flex">
          {binding.event.shapes_xy?.map((coords) => (
            <ShapeSvg
              coords={coords}
            />
          ))}
        </div>
      )}
      <Button onClick={test}>TEST</Button>
    </div>
  );
}
