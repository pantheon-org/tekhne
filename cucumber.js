module.exports = {
  default: {
    paths: ["features/**/*.feature"],
    require: ["features/step-definitions/**/*.ts"],
    requireModule: ["tsx"],
    format: ["progress"],
  },
};
