import type { Meta, StoryObj } from "@storybook/react";
import { ModifiersSelector } from "./ModifiersSelector";
import { useState } from "react";
import { ModifierType } from "./config.type";

const meta = {
  title: "Selectors/ModifiersSelector",
  component: ModifiersSelector,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: { setModifiers: { action: "setModifiers" } },
} satisfies Meta<typeof ModifiersSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const ShiftLeft: Story = { args: { modifiers: ["ShiftLeft"] } };
export const ShiftLeftControlLeft: Story = {
  args: { modifiers: ["ShiftLeft", "ControlLeft"] },
};

export const Empty: Story = { args: { modifiers: [] } };

export const Test = () => {
  const [modifiers, setModifiers] = useState<ModifierType[]>([]);
  return (
    <div>
      <ModifiersSelector {...{ modifiers, setModifiers }} />
    </div>
  );
};
