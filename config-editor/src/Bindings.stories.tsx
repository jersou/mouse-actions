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
  cmd: ["cmd", "cmd", "cmd"],
  comment: "comment",
  event: {
    event_type: "Click",
    button: "Right",
    modifiers: ["ShiftLeft", "Alt"],
    edges: ["Left", "Top"],
  },
};

export const Test1: Story = { args: { binding: binding1 } };

export const Test = () => {
  const [binding, setBinding] = useState<BindingType>(binding1);
  return <Binding binding={binding} setBinding={(b) => setBinding(b)} />;
};
