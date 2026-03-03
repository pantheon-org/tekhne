# Library Mode Guide

Build Nx libraries with Vite while preserving consumer flexibility.

## Minimal Library Mode

```ts
build: {
  outDir: '../../dist/libs/my-lib',
  lib: {
    entry: 'src/index.ts',
    name: 'MyLib',
    fileName: 'index',
    formats: ['es'],
  },
  rollupOptions: {
    external: ['react', 'react-dom', 'react/jsx-runtime'],
  },
}
```

## Multi-Format Output

```ts
build: {
  lib: {
    entry: 'src/index.ts',
    name: 'MyLib',
    fileName: (format) => `index.${format}.js`,
    formats: ['es', 'cjs'],
  },
}
```

## DTS Plugin Setup

Install:

```bash
bun add -d vite-plugin-dts
```

Fast mode (type checks elsewhere):

```ts
import dts from 'vite-plugin-dts';
import { join } from 'path';

plugins: [
  dts({
    entryRoot: 'src',
    tsConfigFilePath: join(__dirname, 'tsconfig.lib.json'),
    skipDiagnostics: true,
  }),
]
```

Strict mode (block on type errors):

```ts
plugins: [
  dts({
    root: '../../',
    entryRoot: 'libs/my-lib/src',
    tsConfigFilePath: 'libs/my-lib/tsconfig.lib.json',
    include: ['libs/my-lib/src/**/*.ts'],
    outputDir: 'dist/libs/my-lib',
    skipDiagnostics: false,
  }),
]
```

## Externalization Rules

Always externalize:

- Peer dependencies
- Runtime deps expected from consumer environment

Pattern:

```ts
rollupOptions: {
  external: ['react', 'react-dom', /^@my-org\//],
}
```

Avoid:

- Shipping bundled React into publishable UI libraries
- Omitting `external` entirely for shared libraries
