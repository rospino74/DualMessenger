module.exports = {
  theme: {
    extend: {},
  },
  content: ['./src/**/*.{svelte,js,ts}'],
  plugins: [
    require('@tailwindcss/typography'),
    require('daisyui')
  ],
  daisyui: {
    themes: ["emerald", "dark"],
  },
};
