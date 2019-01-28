module.exports = {
  root: true,
  env: {
    browser: true,
    node: true
  },
  extends: ["plugin:vue/essential", 'eslint:recommended'],
  plugins: ['prettier', 'vue'],
  rules: {
    "no-console": process.env.NODE_ENV === "production" ? "error" : "off",
    "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off"
  }
};
