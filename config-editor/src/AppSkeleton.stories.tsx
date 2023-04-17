import type { Meta, StoryObj } from "@storybook/react";
import { AppSkeleton } from "./AppSkeleton";

const meta = {
  title: "Selectors/AppSkeleton",
  component: AppSkeleton,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: {},
} satisfies Meta<typeof AppSkeleton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Skeleton: Story = {};
