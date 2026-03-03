# Design System / UI Package Template

Use this template when documenting design system or UI component packages.

**MANDATORY: Before using this template, analyze the current project to identify:**
1. **Component library location** - Run: `find . -name "components" -type d | head -5`
2. **UI framework** - Run: `grep -r "react\|vue\|angular\|svelte" package.json | head -3`
3. **Styling approach** - Run: `find . -name "*.css" -o -name "*.scss" -o -name "*.styled.*" | head -5`
4. **Build tool** - Run: `ls -la *storybook* vite.config.* webpack.config.* rollup.config.* 2>/dev/null`
5. **Design tokens** - Run: `find . -name "*token*" -o -name "*theme*" | grep -v node_modules | head -5`

## Adaptive Template Structure

```markdown
## Design System
- Components: `[DETECTED_COMPONENTS_PATH]/**`
- Design tokens: `[DETECTED_TOKENS_PATH]` OR "No design tokens found"
- [COMPONENT_GALLERY_COMMAND] OR "No component gallery setup found"
- Examples:
  - [EXAMPLE_COMPONENT_1]: `[ACTUAL_PATH_FROM_PROJECT]`
  - [EXAMPLE_COMPONENT_2]: `[ACTUAL_PATH_FROM_PROJECT]`
```

## Detection Instructions

### Component Organization
**Before documenting**, run discovery commands to identify:
- Components directory structure (don't assume `src/components/`)
- Design token files (could be `.json`, `.js`, `.ts`, CSS variables)
- Theme configuration location
- Styling approach used in the project

### Development Commands
**Analyze package.json scripts** to identify:
- Storybook command: `grep -r "storybook" package.json` 
- Build command: `grep -r "build.*ui\|build.*components" package.json`
- Test command: `grep -r "test.*components\|test.*ui" package.json`

### Technology-Specific Patterns
**React Projects**: Look for `.tsx`, styled-components, emotion
**Vue Projects**: Look for `.vue`, scoped styles  
**Angular Projects**: Look for `.component.ts`, Angular Material
**Web Components**: Look for custom element definitions

## Usage Pattern Examples

### Import Patterns (Adapt Based on Actual Project)
```typescript
// Example based on what you find in existing files
import { [ComponentName] } from '[ACTUAL_IMPORT_PATH]';
```

### Component API Design (Copy from Existing)
- Find the most well-structured component in the project
- Copy its prop patterns and API design
- Reference it as the standard to follow

## Anti-Patterns for Design Systems
- Don't assume React/TypeScript (check what's actually used)
- Don't hardcode paths without verification
- Don't reference Storybook if it doesn't exist
- Don't document patterns not used in this specific project