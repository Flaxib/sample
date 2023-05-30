import Solid from "vite-plugin-solid"

module.exports = {
  stories: ["../src/**/*.stories.mdx", "../src/**/*.stories.@(js|jsx|ts|tsx)"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
  ],
  framework: "@storybook/html-vite",
  core: {
    builder: "@storybook/builder-vite",
  },
  features: {
    storyStoreV7: true,
  },
  docs: {
    autodocs: "tag",
  },
  
  // Add solid plugin here
  async viteFinal(config, { configType }) {
    config.plugins.unshift(Solid({ hot: false }));

    return config;
  },
};