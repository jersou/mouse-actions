function edgesEq(edges1: string[], edges2: string[]) {
  return (
    edges1.length === edges2.length && edges1.every((e) => edges2.includes(e))
  );
}

export function ScreenEdges({
  edges,
  setEdges,
}: {
  edges: string[];
  setEdges?: (edges: string[]) => unknown;
}) {
  const EdgeArea = ({ areaEdges, d }: { areaEdges: string[] }) => {
    return (
      <path
        onClick={() =>
          edgesEq(edges, areaEdges) ? setEdges?.([]) : setEdges?.(areaEdges)
        }
        d={d}
        fill={edgesEq(edges, areaEdges) ? "#0f0" : "#da0000"}
        stroke="#4d4d4d"
        strokeDashoffset="3.4"
        strokeLinecap="round"
        strokeWidth=".146"
      />
    );
  };

  return (
    <svg
      width="178"
      height="123"
      version="1.1"
      viewBox="0 0 53.5 37"
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs id="defs12">
        <linearGradient
          id="b"
          x1="86.7"
          x2="86.7"
          y1="209"
          y2="213"
          gradientTransform="translate(-59.8 -177)"
          gradientUnits="userSpaceOnUse"
        >
          <stop id="stop2" stopColor="#b8b8b8" offset="0" />
          <stop id="stop4" stopColor="#f1f1f1" offset="1" />
        </linearGradient>
        <radialGradient
          id="a"
          cx="71.5"
          cy="179"
          r="25.1"
          gradientTransform="matrix(1.64 .758 -.348 .756 -45.8 -182)"
          gradientUnits="userSpaceOnUse"
        >
          <stop id="stop7" stopColor="#c9efff" offset="0" />
          <stop id="stop9" stopColor="#29a4f2" offset="1" />
        </radialGradient>
      </defs>
      <path
        d="m34.7 31.1c0 5.12 4.95 1.51 4.95 5.76h-25.8c0-4.26 4.95-0.646 4.95-5.76"
        color="#000000"
        fill="url(#b)"
        stroke="#4d4d4d"
        strokeDashoffset="3.4"
        strokeLinecap="round"
        strokeWidth=".2"
      />
      <rect
        x=".1"
        y=".1"
        width="53.3"
        height="31.9"
        rx="2"
        ry="2"
        color="#000000"
        fill="#e6e6e6"
        stroke="#4d4d4d"
        strokeDashoffset="3.4"
        strokeLinecap="round"
        strokeWidth=".2"
      />
      <rect
        x="1.8"
        y="2.1"
        width="49.9"
        height="28.1"
        color="#000000"
        fill="url(#a)"
        stroke="#4d4d4d"
        strokeDashoffset="3.4"
        strokeLinecap="round"
        strokeWidth=".2"
      />
      <EdgeArea
        areaEdges={["Top", "Left"]}
        d="m6.83 2.1a5.03 5.03 0 0 1-5.03 5.03v-5.03z"
      />
      <EdgeArea
        areaEdges={["Top", "Right"]}
        d="m46.7 2.1a5.03 5.03 0 0 0 5.03 5.03v-5.03z"
      />
      <EdgeArea
        areaEdges={["Bottom", "Left"]}
        d="m6.83 30.2a5.03 5.03 0 0 0-5.03-5.03v5.03z"
      />
      <EdgeArea
        areaEdges={["Bottom", "Right"]}
        d="m46.7 30.2a5.03 5.03 0 0 1 5.03-5.03v5.03z"
      />
      <EdgeArea
        areaEdges={["Top"]}
        d="m13.7 7.13c-2.78 0-5.03-2.25-5.03-5.03h36.2c-1e-6 2.78-2.25 5.03-5.03 5.03z"
      />
      <EdgeArea
        areaEdges={["Bottom"]}
        d="m13.7 25.2c-2.78 0-5.03 2.25-5.03 5.03h36.2c-1e-6 -2.78-2.25-5.03-5.03-5.03z"
      />
      <EdgeArea
        areaEdges={["Left"]}
        d="m6.83 13.6c-3e-6 -2.78-2.25-5.03-5.03-5.03v15.1c2.78 0 5.03-2.25 5.03-5.03z"
      />
      <EdgeArea
        areaEdges={["Right"]}
        d="m46.7 13.6c0-2.78 2.25-5.03 5.03-5.03v15.1c-2.78 0-5.03-2.25-5.03-5.03z"
      />
    </svg>
  );
}
