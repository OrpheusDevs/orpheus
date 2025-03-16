// tailwind.config.js
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        "deep-blue": "#0a0a2a",
        "dark-purple": "#1a1a4a",
        "tech-purple": {
          100: "#e0d7ff",
          200: "#c2b0ff",
          300: "#a389ff",
          400: "#8462ff",
          500: "#653bff",
          600: "#5130cc",
          700: "#3d2499",
          800: "#291966",
          900: "#140f33",
        },
      },
      animation: {
        "pulse-slow": "pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite",
      },
      boxShadow: {
        neon: '0 0 5px theme("colors.purple.500"), 0 0 20px theme("colors.purple.500")',
      },
      backgroundImage: {
        "tech-gradient": "linear-gradient(to bottom right, #0a0a2a, #1a1a4a)",
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
