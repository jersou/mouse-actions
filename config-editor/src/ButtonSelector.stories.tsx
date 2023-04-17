import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { ButtonType } from "./config.type";
import { ButtonSelector } from "./ButtonSelector";

const meta = {
  title: "Selectors/ButtonSelector",
  component: ButtonSelector,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: { setButton: { action: "setButton" } },
} satisfies Meta<typeof ButtonSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Right: Story = { args: { button: "Right" } };
export const WheelDown: Story = { args: { button: "WheelDown" } };

export const Test = () => {
  const [button, setButton] = useState<ButtonType>("Right");
  return (
    <div>
      <ButtonSelector {...{ button, setButton }} />
    </div>
  );
};
