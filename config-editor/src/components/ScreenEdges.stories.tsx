import type { Meta, StoryObj } from "@storybook/react";
import { ScreenEdges } from "./ScreenEdges";

const meta = {
  title: "ScreenEdges",
  component: ScreenEdges,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
} satisfies Meta<typeof ScreenEdges>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Left: Story = { args: { edges: ["Left"] } };
export const TopLeft: Story = { args: { edges: ["Top", "Left"] } };

export const Empty: Story = { args: { edges: [] } };

export const Test = () => (
  <div>
    →<ScreenEdges edges={["Top", "Left"]} />←
  </div>
);
