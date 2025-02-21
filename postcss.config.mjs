const config = {
  plugins: {
    "@tailwindcss/postcss": {
      // https://github.com/vercel/next.js/issues/75817
      optimize: { minify: false },
    },
  },
}

export default config
