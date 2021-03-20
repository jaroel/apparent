// const defaultTheme = require('tailwindcss/defaultTheme');

module.exports = {
  purge: {
    enabled: true,
    content: [
      // path is relative to where `cargo make build_css` is run.
      './client/src/**/*.rs',
      './client/src/*.rs',
      './client/index.html',
    ]
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
