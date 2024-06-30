/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./dashboard/templates/**/*.{html,js,j2}"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["fantasy"],
  },
};
