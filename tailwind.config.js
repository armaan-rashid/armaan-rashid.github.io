/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  theme: {
    extend: {
      backgroundImage: {
        "pearl-string": "url('public/PearlString.svg')",
      },
      colors: {
        background: "#f7f0dd",
        haskell: "#bb89fb",
        swift: "#FF3333",
        python: "#22B6F5",
        rust: "#FF8C38",
        codeview: "#1C1A1A",
        light: "#D9D9D9",
      },
      fontFamily: {
        lexend: ["Lexend", "sans-serif"],
      },
    },
  },
  plugins: [],
};
