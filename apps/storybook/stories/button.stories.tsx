import type { Meta, StoryObj } from "@storybook/react";
import { Button } from "@framer-university/ui";

const meta = {
  title: "Components/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    intent: {
      control: { type: "radio" },
      options: ["primary", "secondary"],
    },
    size: {
      control: { type: "radio" },
      options: ["sm", "md", "lg"],
    },
  },
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof Button>;

export const Primary: Story = {
  args: {
    children: "Primary",
    intent: "primary",
    size: "md",
  },
};

export const Secondary: Story = {
  args: {
    children: "Secondary",
    intent: "secondary",
    size: "md",
  },
};
