import type { Meta, StoryObj } from "@storybook/react";
import { Binding } from "./Binding";
import { useState } from "react";
import { BindingType } from "./config.type";

const meta = {
  title: "Binding",
  component: Binding,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: { setBinding: { action: "setBinding" } },
} satisfies Meta<typeof Binding>;

export default meta;
type Story = StoryObj<typeof meta>;

const binding1: BindingType = {
  cmd_str: "cmd cmd cmd",
  comment: "comment",
  event: {
    event_type: "Click",
    button: "Right",
    modifiers: ["ShiftLeft", "Alt"],
    edges: ["Left", "Top"],
    shapes_xy: [
      [1000, 400, 800, 200, 400, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000, 1000],
      [400, 1000, 200, 800, 0, 400, 0, 0, 0, 0, 0, 0, 0, 0, 1000, 1000],
    ],
  },
};

export const Test1: Story = { args: { binding: binding1 } };

export const Test = () => {
  const [binding, setBinding] = useState<BindingType>(binding1);
  return (
    <Binding
      binding={binding}
      setBinding={(b) => setBinding(b)}
      addBinding={() => console.log("addBinding")}
      deleteBinding={() => console.log("deleteBinding")}
    />
  );
};
