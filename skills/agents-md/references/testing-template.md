# Testing Template

Use this template when documenting testing setup and patterns for packages.

**MANDATORY: Before using this template, analyze the current project to identify:**
1. **Testing framework** - Run: `grep -r "jest\|vitest\|mocha\|jasmine\|pytest\|go.*test\|cargo.*test" package.json pyproject.toml Cargo.toml go.mod | head -3`
2. **Test file patterns** - Run: `find . -name "*.test.*" -o -name "*.spec.*" | head -5`
3. **E2E framework** - Run: `grep -r "playwright\|cypress\|puppeteer\|selenium" package.json | head -3`
4. **Test directories** - Run: `find . -name "*test*" -type d | grep -v node_modules | head -5`
5. **Config files** - Run: `ls -la jest.config.* vitest.config.* playwright.config.* cypress.config.* pytest.ini 2>/dev/null`

## Adaptive Template Structure

```markdown
## Testing
- Framework: [DETECTED_TEST_FRAMEWORK] ([Jest/Vitest/pytest/etc])
- Unit: `[DETECTED_TEST_PATTERN]` ([*.test.ts/test_*.py/etc]) 
- Integration: `[DETECTED_INTEGRATION_PATH]/**` OR "No integration tests found"
- E2E: `[DETECTED_E2E_PATH]/**` ([DETECTED_E2E_FRAMEWORK]) OR "No E2E tests found"
- Run single: `[DETECTED_SINGLE_TEST_COMMAND] [file-path]`
- Mocks: `[DETECTED_MOCK_PATH]/**` OR "Mock location not standardized"
```

## Detection Instructions

### Framework-Specific Patterns
**Jest**: Look for `jest.config.js`, tests with `.test.js/.spec.js`
**Vitest**: Look for `vitest.config.ts`, similar patterns to Jest
**Mocha**: Look for `mocha.opts`, test directory structure
**pytest**: Look for `pytest.ini`, `test_*.py` or `*_test.py` files
**Go**: Look for `*_test.go` files, `go test` commands
**Rust**: Look for `#[test]` annotations in `src/` or `tests/`

### Test Organization (Copy from Actual Project)
**Find existing patterns:**
- Where are unit tests actually located?
- How are integration tests organized?
- What test file naming convention is used?
- Where are test utilities/helpers located?

### Commands and Scripts (Extract from package.json/Makefile)
**Find actual commands used:**
- Test runner commands from `package.json` scripts
- Coverage generation commands
- Watch mode setup
- Specific test file execution pattern

## Technology-Specific Adaptations

### JavaScript/TypeScript (Jest/Vitest)
- Test setup files location
- Mock patterns in use
- Coverage configuration
- Test environment setup

### Python (pytest)
- Fixture definitions and location  
- Conftest.py usage
- Test markers and parametrization
- Virtual environment handling

### Go
- Test package organization
- Table-driven test patterns
- Benchmark tests (if present)
- Test helper functions

### Other Languages
**Adapt based on what's detected in the project**

## Testing Patterns (Extract from Existing Tests)
**Before documenting, examine existing tests for:**
- Setup/teardown patterns actually used
- Mocking approach and conventions
- Test data generation methods
- Assertion patterns and styles

## Critical Warnings (Universal)
- NEVER run tests against production data or APIs
- NEVER commit real API keys or credentials in test files
- NEVER rely on external services in unit tests (use mocks)
- NEVER assume testing framework without detection

## Anti-Patterns for Testing Documentation
- Don't explain testing concepts or philosophies
- Don't assume specific framework without verification
- Don't document assertion syntax basics
- Don't duplicate framework documentation that's already available