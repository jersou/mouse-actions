export const Buttons = [
  "Left",
  "Right",
  "iddle",
  "Side",
  "Extra",
  "Forward",
  "Back",
  "Task",
  "Trigger",
  "Thumb",
  "Thumb2",
  "WheelUp",
  "WheelDown",
  "Unknown",
  "None",
] as const;
export type ButtonType = (typeof Buttons)[number];

export const Modifiers = [
  "ShiftLeft",
  "ShiftRight",
  "ControlLeft",
  "ControlRight",
  "MetaLeft",
  "Alt",
  "AltGr",
] as const;

export type ModifierType = (typeof Modifiers)[number];

export const Events = ["Press", "Release", "Click", "Shape"] as const;
export type EventTypeType = (typeof Events)[number];

export const Edges = ["Top", "Right", "Bottom", "Left"] as const;
export type EdgeType = (typeof Edges)[number];

export type EventType = {
  button: ButtonType;
  modifiers?: ModifierType[];
  event_type: EventTypeType;
  edges?: EdgeType[];
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
