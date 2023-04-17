import type { Meta, StoryObj } from "@storybook/react";
import App from "./App";

const meta = {
  title: "App",
  component: App,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
} satisfies Meta<typeof App>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Empty: Story = { args: {} };

//   </div>
// };
