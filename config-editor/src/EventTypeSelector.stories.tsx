import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { EventTypeType } from "./config.type";
import { EventTypeSelector } from "./EventTypeSelector";

const meta = {
  title: "Selectors/EventTypeSelector",
  component: EventTypeSelector,
  // This component will have an automatically generated Autodocs entry: https://storybook.js.org/docs/react/writing-docs/autodocs
  tags: ["autodocs"],
  argTypes: { setEventType: { action: "setEventType" } },
} satisfies Meta<typeof EventTypeSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Release: Story = { args: { eventType: "Release" } };
export const Press: Story = { args: { eventType: "Press" } };

export const Test = () => {
  const [eventType, setEventType] = useState<EventTypeType>("Click");
  return (
    <div>
      <EventTypeSelector {...{ eventType, setEventType }} />
    </div>
  );
};
