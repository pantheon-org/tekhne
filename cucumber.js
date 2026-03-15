module.exports = {
  default: {
    paths: ["cli/features/**/*.feature"],
    require: ["cli/features/step-definitions/**/*.ts"],
    requireModule: ["tsx"],
    format: ["progress"],
  },
};
