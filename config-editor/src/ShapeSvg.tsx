export function ShapeSvg({ coords }: { coords: number[] }) {
  const lines = [];
  let x1 = coords[0];
  let y1 = coords[1];
  // TODO add arrow
  for (let i = 2; i < coords.length; i += 2) {
    const x2 = coords[i];
    const y2 = coords[i + 1];
    lines.push(
      <line key={i}
        x1={x1}
        y1={y1}
        x2={x2}
        y2={y2}
        stroke={`hsl(${Math.round((140 * i) / coords.length)}deg 100% 60%)`}
        strokeWidth="50"
      />
    );
    x1 = x2;
    y1 = y2;
  }
  if (coords.length > 10) {
    lines.push(
      <line key="z"
        markerEnd="url(#arrow)"
        x1={coords[coords.length - 10]}
        y1={coords[coords.length - 9]}
        x2={coords[coords.length - 2]}
        y2={coords[coords.length - 1]}
        fill="none"
        strokeWidth="30"
      />
    );
  }
  return (
    <svg
      viewBox="-100 -100 1200 1200"
      width="150"
      height="150"
      style={{ border: "black solid 1px", borderRadius: 10, margin: 4 }}
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs>
        <marker
          id="arrow"
          viewBox="0 0 10 10"
          refX="5"
          refY="5"
          fill="hsl(140deg 100% 60%)"
          markerWidth="4"
          markerHeight="4"
          orient="auto-start-reverse"
        >
          <path d="M 0 0 L 10 5 L 0 10 z" />
        </marker>
      </defs>
      {lines}
    </svg>
  );
}
