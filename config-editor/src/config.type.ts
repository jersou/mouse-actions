export type ButtonType =
  | "eft"
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

export type ModifierType =
  | "ShiftLeft"
  | "ShiftRight"
  | "ControlLeft"
  | "ControlRight"
  | "MetaLeft"
  | "Alt"
  | "AltGr";
export type EventTypeType = "Press" | "Release" | "Click";
export type Edge = "Top" | "Right" | "Bottom" | "Left";

export type EventType = {
  button: ButtonType;
  modifiers?: ModifierType[];
  event_type: EventTypeType;
  edges?: Edge[];
  shapes_xy?: [number[]];
};

export type BindingType = {
  comment: string;
  cmd: string[];
  event: EventType;
};

export type ConfigType = {
  shape_button: ButtonType;
  bindings: BindingType[];
};
