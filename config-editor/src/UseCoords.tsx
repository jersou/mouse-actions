import {useEffect, useRef, useState} from "react";
import {Point} from "./Config";

export function useCoords(listenerEnable: boolean) {
  const [coords, setCoords] = useState<number[]>([]);
  const pointsHistory = useRef<Point[]>([]);
  const mouseState = useRef(false);

  const mousemove = (event: any) => {
    if (mouseState.current) {
      pointsHistory.current.push({x: event.x, y: event.y});
    }
  };
  const mousedown = (event: any) => {
    mouseState.current = true;
    pointsHistory.current = [];
  };
  const mouseup = (event: any) => {
    mouseState.current = false;
    if (pointsHistory.current.length > 10) {
      const raw: { x: number; y: number }[] = pointsHistory.current;
      const minX = Math.min(...raw.map((point) => point.x));
      const minY = Math.min(...raw.map((point) => point.y));
      const maxX = Math.max(...raw.map((point) => point.x));
      const maxY = Math.max(...raw.map((point) => point.y));
      const width = maxX - minX;
      const height = maxY - minY;
      const size = Math.max(width, height);
      const normalizedCoords: number[] = [];
      for (const {x, y} of raw) {
        normalizedCoords.push(Math.round(((x - minX) * 1000) / size));
        normalizedCoords.push(Math.round(((y - minY) * 1000) / size));
      }
      setCoords(normalizedCoords);
    }
  };

  useEffect(() => {
    if (listenerEnable) {
      window.addEventListener("mousemove", mousemove);
      window.addEventListener("mousedown", mousedown);
      window.addEventListener("mouseup", mouseup);
      return () => {
        window.removeEventListener("mousemove", mousemove);
        window.removeEventListener("mousedown", mousedown);
        window.removeEventListener("mouseup", mouseup);
      };
    }
  }, [listenerEnable]);

  return coords;
}
