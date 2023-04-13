export function ScreenEdges({ edges }: { edges: string[] }) {
  if (edges.length == 1) {
    switch (edges[0]) {
      case "Left":
        return <img src="/screen-left.svg" width="150" alt="left" />;
      case "Right":
        return <img src="/screen-right.svg" width="150" alt="right" />;
      case "Top":
        return <img src="/screen-top-left.svg" width="150" alt="top" />;
      case "Bottom":
        return <img src="/screen-bottom.svg" width="150" alt="bottom" />;
    }
  } else if (edges.length == 2) {
    if (edges.includes("Top")) {
      if (edges.includes("Left")) {
        return <img src="/screen-top-left.svg" width="150" alt="top" />;
      } else if (edges.includes("Right")) {
        return <img src="/screen-top-right.svg" width="150" alt="top" />;
      }
    } else if (edges.includes("Bottom")) {
      if (edges.includes("Left")) {
        return <img src="/screen-bottom-left.svg" width="150" alt="top" />;
      } else if (edges.includes("Right")) {
        return <img src="/screen-bottom-right.svg" width="150" alt="top" />;
      }
    }
  }
  // return <img src="/screen.svg" width="150" alt="top" />;
  return null;
}
