import type { StorybookConfig } from "@storybook/vue3-vite"
import tailwindcss from "@tailwindcss/vite"

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.ts"],
  addons: ["@chromatic-com/storybook", "@storybook/addon-vitest"],
  framework: {
    name: "@storybook/vue3-vite",
    options: {
      docgen: "vue-component-meta",
    },
  },
  core: {
    disableTelemetry: true,
  },
}

// Ensure the Tailwind Vite plugin is applied to Storybook's Vite config
config.viteFinal = async (viteConfig) => {
  viteConfig.plugins = [...(viteConfig.plugins || []), tailwindcss()]
  return viteConfig
}

export default config
