/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        clifford: '#da373d',
        accent: '#706CF6',
        error: '#F2BAB9',
        success: '#D2ECAE',
        warn: '#F9F4BC',
        neutral: '#CEEFF4',
      }
    },
  },
  plugins: [],
}
