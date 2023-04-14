export type Button =
  "eft"
  | "Right"
  | "iddle"
  | "Side"
  | "Extra"
  | "Forward"
  | "Back"
  | "Task"
  | "Trigger"
  | "Thumb"
  | "Thumb2"
  | "WheelUp"
  | "WheelDown"
  | "Unknown"
  | "None";

export type Modifier = "ShiftLeft" | "ShiftRight" | "ControlLeft" |
  "ControlRight" | "MetaLeft" | "Alt" | "AltGr";
export type EventType = "Press" | "Release" | "Click";
export type Edge = "Top" | "Right" | "Bottom" | "Left";

export type Event = {
  button: Button,
  modifiers?: Modifier[],
  event_type: EventType,
  edges?: Edge[],
  shapes_xy?: [number[]]
};

export type Binding = {
  "comment": string,
  "cmd": string[],
  "event": Event,
}

export type ConfigType = {
  shape_button: Button;
  bindings: Binding[]
}
