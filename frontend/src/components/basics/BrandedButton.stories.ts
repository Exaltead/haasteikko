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
    styling: {
      isPill: false,
      backgroundColor: "primary",
      bold: false,
      iconColor: "white",
    }
  }
} satisfies Meta<typeof BrandedButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Primary: Story = {
  args: {

  }
}
