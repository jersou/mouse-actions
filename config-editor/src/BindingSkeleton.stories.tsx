import type { Meta, StoryObj } from "@storybook/react";
import { BindingSkeleton } from "./BindingSkeleton";

const meta = {
  title: "Selectors/BindingSkeleton",
  component: BindingSkeleton,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: {},
} satisfies Meta<typeof BindingSkeleton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Skeleton: Story = {};
