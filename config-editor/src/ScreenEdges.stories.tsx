import type {Meta, StoryObj} from "@storybook/react";
import {ScreenEdges} from "./ScreenEdges";
import {useState} from "react";

const meta = {
  title: "ScreenEdges",
  component: ScreenEdges,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: {setEdges: {action: 'setEdges'}},
} satisfies Meta<typeof ScreenEdges>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Left: Story = {args: {edges: ["Left"]}};
export const TopLeft: Story = {args: {edges: ["Top", "Left"]}};

export const Empty: Story = {args: {edges: []}};

export const Test = () => {
  const [edges, setEdges] = useState([])
  return <div>
    <ScreenEdges {...{edges, setEdges}}/>
  </div>
};
