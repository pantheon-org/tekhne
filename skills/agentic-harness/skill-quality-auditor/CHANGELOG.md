# Changelog

## [0.3.0](https://github.com/pantheon-org/tekhne/compare/v0.2.0...v0.3.0) (2026-07-23)


### Features

* **agentic-context:** add triage-paper and triage-tool skills ([#57](https://github.com/pantheon-org/tekhne/issues/57)) ([bc16396](https://github.com/pantheon-org/tekhne/commit/bc1639618555a64ab0543211befa69aeb1cd886b))
* **docs:** add Astro Starlight docs site with GitHub Pages deployment ([#13](https://github.com/pantheon-org/tekhne/issues/13)) ([a2f3679](https://github.com/pantheon-org/tekhne/commit/a2f36795b9eaed06b71efda9df41f53877922a8b))
* **project-mgmt:** add implementation-planner skill ([#5](https://github.com/pantheon-org/tekhne/issues/5)) ([834b884](https://github.com/pantheon-org/tekhne/commit/834b88482e25e5ac0cd611d0de06d1eefcca3d4f))
* **project-mgmt:** consolidate planning skills into planning-toolkit tile ([#55](https://github.com/pantheon-org/tekhne/issues/55)) ([2f91c1b](https://github.com/pantheon-org/tekhne/commit/2f91c1b4bf7ed616cd32cadd7031d6199686e9ce))
* **skill-quality-auditor:** add D9 Eval Validation dimension and scale to 140-point framework ([954f964](https://github.com/pantheon-org/tekhne/commit/954f9645dca8d13323d196c2b8d180ea281d5876))
* **skill-quality-auditor:** add kitchen-sink anti-pattern and improve to A-grade (113/120) ([874f7e2](https://github.com/pantheon-org/tekhne/commit/874f7e28b71e81b240baa05c3df7dc8a55e41506))
* **skill-quality-auditor:** add self-containment penalties and script portability bonus to D4 ([871bcb2](https://github.com/pantheon-org/tekhne/commit/871bcb29ba6f1ec0d5a704a1203b3c27651aebe2))
* **skill-quality-auditor:** enforce References table standard + add eval suites across 40+ skills ([#26](https://github.com/pantheon-org/tekhne/issues/26)) ([2d4c8cf](https://github.com/pantheon-org/tekhne/commit/2d4c8cfb0a57290e20e70e72186b8021bf802687))
* **skill-quality-auditor:** improve D4/D5/D7/D8 dimensions and fix 9-dimension references ([73270d4](https://github.com/pantheon-org/tekhne/commit/73270d42ad9a0ab153272de2914be2f242f6a029))
* **skill-validator-rs:** port validate-skill-artifacts; retire check-consistency (Wave 2) ([#211](https://github.com/pantheon-org/tekhne/issues/211)) ([9a33d4e](https://github.com/pantheon-org/tekhne/commit/9a33d4e218a85f749d63c6de9b144c396d1b41d0))
* **software-engineering:** split software-design-principles into 4 focused skills ([7766492](https://github.com/pantheon-org/tekhne/commit/77664920fffe77becb04b991af6d04a085b4959c))
* **tools:** add skill-auditor Go binary (replaces evaluate.sh) ([#90](https://github.com/pantheon-org/tekhne/issues/90)) ([13307fa](https://github.com/pantheon-org/tekhne/commit/13307fa3166ac9d6d8f8c3ebf249911d3c03554d))


### Bug Fixes

* **agents-md:** improve skill quality B+ → A+ (121 → 133/140) ([#34](https://github.com/pantheon-org/tekhne/issues/34)) ([ca6ae79](https://github.com/pantheon-org/tekhne/commit/ca6ae79f9ec7bda40f2c986e18b587f6cd2dd2a3))
* **skill-quality-auditor:** enforce assets/ subdirectory conventions ([bc16396](https://github.com/pantheon-org/tekhne/commit/bc1639618555a64ab0543211befa69aeb1cd886b))
* **skill-quality-auditor:** enforce lazy loading of references in D5 ([#30](https://github.com/pantheon-org/tekhne/issues/30)) ([9ceb8b4](https://github.com/pantheon-org/tekhne/commit/9ceb8b493ac887e4e3eeb5c2bb966a1e18d54c4c))
* **skill-quality-auditor:** use || true to suppress grep -c non-zero exit under set -e ([#46](https://github.com/pantheon-org/tekhne/issues/46)) ([8a931e2](https://github.com/pantheon-org/tekhne/commit/8a931e232c59c4fafadf31333f798a18ccf3f69a))
* **software-engineering:** complete design-principles migration from software-design-principles ([fb797b0](https://github.com/pantheon-org/tekhne/commit/fb797b012f5756fcb2445747671ac0db19431e6f))

## [0.2.0](https://github.com/pantheon-org/tekhne/compare/v0.1.4...v0.2.0) (2026-05-15)


### Features

* **agentic-context:** add triage-paper and triage-tool skills ([#57](https://github.com/pantheon-org/tekhne/issues/57)) ([27e33ef](https://github.com/pantheon-org/tekhne/commit/27e33ef245f1bb6a6830ff1e1c898142dc0cdf9c))
* **docs:** add Astro Starlight docs site with GitHub Pages deployment ([#13](https://github.com/pantheon-org/tekhne/issues/13)) ([54ef78a](https://github.com/pantheon-org/tekhne/commit/54ef78a5c7f9aa9eba619a34ad9d577bb48a2f37))
* **project-mgmt:** add implementation-planner skill ([#5](https://github.com/pantheon-org/tekhne/issues/5)) ([a79bc68](https://github.com/pantheon-org/tekhne/commit/a79bc6893e41932ef6488f1f8397181e467c5381))
* **project-mgmt:** consolidate planning skills into planning-toolkit tile ([#55](https://github.com/pantheon-org/tekhne/issues/55)) ([68b5b6b](https://github.com/pantheon-org/tekhne/commit/68b5b6bbdc5bc13391d50a6df3916fada0e8315e))
* **skill-quality-auditor:** add D9 Eval Validation dimension and scale to 140-point framework ([d46339c](https://github.com/pantheon-org/tekhne/commit/d46339c2e42104fb41e4572bf47b17a0cb46f2bb))
* **skill-quality-auditor:** add kitchen-sink anti-pattern and improve to A-grade (113/120) ([6bd32e3](https://github.com/pantheon-org/tekhne/commit/6bd32e3f271c440756b3b587a734ad5f355a24a0))
* **skill-quality-auditor:** add self-containment penalties and script portability bonus to D4 ([ec61433](https://github.com/pantheon-org/tekhne/commit/ec61433af89ff9c60a3320382e9c03d813d6e07a))
* **skill-quality-auditor:** enforce References table standard + add eval suites across 40+ skills ([#26](https://github.com/pantheon-org/tekhne/issues/26)) ([e6da355](https://github.com/pantheon-org/tekhne/commit/e6da355d773aa5646dea9ec6128af0fee0a43ebb))
* **skill-quality-auditor:** improve D4/D5/D7/D8 dimensions and fix 9-dimension references ([b6fd01e](https://github.com/pantheon-org/tekhne/commit/b6fd01e1047827a5cbe250ed98d2693c5d1cb277))
* **software-engineering:** split software-design-principles into 4 focused skills ([c2b21c3](https://github.com/pantheon-org/tekhne/commit/c2b21c3e7dd977d62f3f846463c7fe3d7f8aa96d))
* **tools:** add skill-auditor Go binary (replaces evaluate.sh) ([#90](https://github.com/pantheon-org/tekhne/issues/90)) ([57f5214](https://github.com/pantheon-org/tekhne/commit/57f5214761adbac2bbea8b01695cbc3e7b7f1429))


### Bug Fixes

* **agents-md:** improve skill quality B+ → A+ (121 → 133/140) ([#34](https://github.com/pantheon-org/tekhne/issues/34)) ([2cc3a6b](https://github.com/pantheon-org/tekhne/commit/2cc3a6bd5fb9ffccda960df84cba77bda7b8da3d))
* **skill-quality-auditor:** enforce assets/ subdirectory conventions ([27e33ef](https://github.com/pantheon-org/tekhne/commit/27e33ef245f1bb6a6830ff1e1c898142dc0cdf9c))
* **skill-quality-auditor:** enforce lazy loading of references in D5 ([#30](https://github.com/pantheon-org/tekhne/issues/30)) ([cde25f7](https://github.com/pantheon-org/tekhne/commit/cde25f76c8ac1aaddeda041349622a185da92209))
* **skill-quality-auditor:** use || true to suppress grep -c non-zero exit under set -e ([#46](https://github.com/pantheon-org/tekhne/issues/46)) ([853a600](https://github.com/pantheon-org/tekhne/commit/853a600579a46a46e1abf29f1ffc340924147086))
* **software-engineering:** complete design-principles migration from software-design-principles ([f0ffe05](https://github.com/pantheon-org/tekhne/commit/f0ffe05470325e32474361c026c102e2871f6b08))
