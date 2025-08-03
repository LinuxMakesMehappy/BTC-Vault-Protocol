/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        'vault-primary': '#1a1a1a',
        'vault-secondary': '#2d2d2d',
        'vault-accent': '#f7931a', // Bitcoin orange
        'vault-success': '#10b981',
        'vault-error': '#ef4444',
        'vault-warning': '#f59e0b',
      },
      fontFamily: {
        'mono': ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [],
};