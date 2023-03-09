/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx,css,md,mdx,html,json,scss}',
  ],
  darkMode: 'media',
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Open Sans"', '"Helvetica Neue"', 'sans-serif']
      }
    },
  },
  plugins: [
    require('daisyui')
  ],
  daisyui: {
    darkTheme: 'dark',
    themes: ['light', 'dark']
  }
};
