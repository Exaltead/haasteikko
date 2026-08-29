import type { Meta, StoryObj } from '@storybook/vue3-vite';
import BrandedButton from './BrandedButton.vue';
import { fn } from 'storybook/test';


const meta = {
  title: 'Basics/BrandedButton',
  tags: ['autodocs'],
  component: BrandedButton,
  args: {
    onClick: fn(),
    text: 'Branded Button',
    isSubmitting: false,
    icon: "Author",
    disabled: false,
  }
} satisfies Meta<typeof BrandedButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {
    variant: "primary",
  }
}

export const PrimaryDisabled: Story = {
  args: {
    variant: "primary",
    disabled: true,
  }
}

export const PrimaryWithIconOnly: Story = {
  args: {
    variant: "primary",
    text: "",
    icon: "Plus",
  }
}

export const PrimarySubmitting: Story = {
  args: {
    variant: "primary",
    isSubmitting: true,
  }
}

export const PillButton: Story = {
  args: {
    variant: "primary",
    isPill: true,
  }
}

export const BoldTextButton: Story = {
  args: {
    variant: "primary",
    bold: true,
  }
}

export const Secondary: Story = {
  args: {
    variant: "secondary",
  }
}

export const SecondaryDisabled: Story = {
  args: {
    variant: "secondary",
    disabled: true,
  }
}
