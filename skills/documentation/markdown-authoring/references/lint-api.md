# Markdownlint Programmatic API

## Installation

```bash
npm install markdownlint
```

## Basic Usage

### Callback-Based API

```javascript
const markdownlint = require('markdownlint');

const options = {
  files: ['README.md', 'docs/**/*.md'],
  config: {
    default: true,
    MD013: { line_length: 120 }
  }
};

markdownlint(options, (err, result) => {
  if (err) {
    console.error(err);
    return;
  }
  
  console.log(result.toString());
  // README.md: 23: MD013/line-length Line length [Expected: 120; Actual: 145]
});
```

### Promise-Based API

```javascript
const { markdownlint } = require('markdownlint/promise');

async function lintFiles() {
  const options = {
    files: ['README.md', 'docs/**/*.md'],
    config: {
      default: true,
      MD013: { line_length: 120 }
    }
  };
  
  try {
    const result = await markdownlint(options);
    console.log(result.toString());
  } catch (err) {
    console.error(err);
  }
}

lintFiles();
```

### Synchronous API

```javascript
const { markdownlintSync } = require('markdownlint/sync');

const options = {
  files: ['README.md'],
  config: {
    default: true
  }
};

const result = markdownlintSync(options);
console.log(result.toString());
```

## Options Object

### Basic Options

```javascript
const options = {
  // Files to lint (glob patterns supported)
  files: ['**/*.md'],
  
  // Or strings to lint
  strings: {
    'content1': '# Heading\n\nContent here',
    'content2': '# Another\n\nMore content'
  },
  
  // Configuration object
  config: {
    default: true,
    MD013: { line_length: 120 }
  },
  
  // Custom rules
  customRules: [
    require('./custom-rule')
  ],
  
  // Directories to search for rules
  markdownItPlugins: [
    [require('markdown-it-emoji')]
  ]
};
```

### Advanced Options

```javascript
const options = {
  files: ['README.md'],
  
  config: {
    default: true,
    MD013: false
  },
  
  // Front matter pattern (RegExp or array of RegExp)
  frontMatter: /^---[\s\S]*?---$/,
  
  // Enable/disable inline configuration
  noInlineConfig: false,
  
  // Results version for compatibility
  resultVersion: 3,
  
  // Handle results
  handleRuleFailures: true
};
```

## Working with Results

### Result Object Structure

```javascript
const result = await markdownlint(options);

// result is an object with filename keys:
// {
//   'README.md': [
//     {
//       lineNumber: 23,
//       ruleNames: ['MD013', 'line-length'],
//       ruleDescription: 'Line length',
//       ruleInformation: 'https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md#md013',
//       errorDetail: 'Expected: 120; Actual: 145',
//       errorContext: 'This is a very long line that...',
//       errorRange: [1, 145],
//       fixInfo: null
//     }
//   ]
// }
```

### Processing Results

```javascript
const result = await markdownlint(options);

// Check if any errors found
const hasErrors = Object.keys(result).some(
  file => result[file].length > 0
);

if (hasErrors) {
  console.error('Linting errors found:');
  
  Object.keys(result).forEach(file => {
    const errors = result[file];
    if (errors.length > 0) {
      console.error(`\n${file}:`);
      errors.forEach(error => {
        console.error(
          `  ${error.lineNumber}: ${error.ruleNames[0]} ` +
          `${error.ruleDescription} ${error.errorDetail || ''}`
        );
      });
    }
  });
  
  process.exit(1);
}
```

### Counting Errors

```javascript
const result = await markdownlint(options);

let totalErrors = 0;
const errorsByRule = {};

Object.values(result).forEach(fileErrors => {
  totalErrors += fileErrors.length;
  
  fileErrors.forEach(error => {
    const ruleName = error.ruleNames[0];
    errorsByRule[ruleName] = (errorsByRule[ruleName] || 0) + 1;
  });
});

console.log(`Total errors: ${totalErrors}`);
console.log('Errors by rule:', errorsByRule);
```

## Linting Strings

### Single String

```javascript
const { markdownlint } = require('markdownlint/promise');

async function lintString(content) {
  const result = await markdownlint({
    strings: {
      'content': content
    },
    config: {
      default: true
    }
  });
  
  return result.content || [];
}

const errors = await lintString('# Heading\n\nSome content');
```

### Multiple Strings

```javascript
const result = await markdownlint({
  strings: {
    'file1': '# First\n\nContent',
    'file2': '# Second\n\nMore content',
    'file3': '# Third\n\nEven more'
  },
  config: {
    default: true
  }
});

// Access results by key
console.log(result.file1);
console.log(result.file2);
console.log(result.file3);
```

## Reading Configuration

### Load Config from File

```javascript
const { readConfig } = require('markdownlint/sync');

// Read .markdownlint.json
const config = readConfigSync('.markdownlint.json');

const result = await markdownlint({
  files: ['README.md'],
  config: config
});
```

### Merge Configurations

```javascript
const baseConfig = readConfigSync('.markdownlint.json');
const overrides = {
  MD013: { line_length: 150 }
};

const config = { ...baseConfig, ...overrides };

const result = await markdownlint({
  files: ['README.md'],
  config: config
});
```

## Applying Fixes

### Get Fix Information

```javascript
const result = await markdownlint(options);

Object.keys(result).forEach(file => {
  result[file].forEach(error => {
    if (error.fixInfo) {
      console.log(`${file}:${error.lineNumber} can be auto-fixed`);
    }
  });
});
```

### Apply Fixes

```javascript
const { markdownlint } = require('markdownlint/promise');
const { applyFixes } = require('markdownlint');
const fs = require('fs').promises;

async function lintAndFix(filePath) {
  // Read file
  const content = await fs.readFile(filePath, 'utf8');
  
  // Lint
  const result = await markdownlint({
    strings: {
      [filePath]: content
    },
    config: {
      default: true
    }
  });
  
  // Apply fixes
  const fixed = applyFixes(content, result[filePath]);
  
  // Write back
  await fs.writeFile(filePath, fixed, 'utf8');
  
  console.log(`Fixed ${filePath}`);
}

await lintAndFix('README.md');
```

### Bulk Fix Multiple Files

```javascript
const { markdownlint } = require('markdownlint/promise');
const { applyFixes } = require('markdownlint');
const fs = require('fs').promises;
const glob = require('glob');

async function fixAllFiles(pattern) {
  const files = glob.sync(pattern);
  
  for (const file of files) {
    const content = await fs.readFile(file, 'utf8');
    
    const result = await markdownlint({
      strings: { [file]: content },
      config: { default: true }
    });
    
    if (result[file] && result[file].length > 0) {
      const fixed = applyFixes(content, result[file]);
      
      if (fixed !== content) {
        await fs.writeFile(file, fixed, 'utf8');
        console.log(`Fixed ${file}`);
      }
    }
  }
}

await fixAllFiles('docs/**/*.md');
```

## Custom Rules

### Creating a Custom Rule

```javascript
// custom-no-lorem.js
module.exports = {
  names: ['no-lorem'],
  description: 'Disallow lorem ipsum text',
  tags: ['test'],
  function: (params, onError) => {
    params.tokens.forEach(token => {
      if (token.type === 'inline') {
        const content = token.content.toLowerCase();
        if (content.includes('lorem ipsum')) {
          onError({
            lineNumber: token.lineNumber,
            detail: 'Found "lorem ipsum" placeholder text',
            context: token.line
          });
        }
      }
    });
  }
};
```

### Using Custom Rules

```javascript
const customRule = require('./custom-no-lorem');

const result = await markdownlint({
  files: ['README.md'],
  config: {
    default: true
  },
  customRules: [customRule]
});
```

## Integration Examples

### Express.js API Endpoint

```javascript
const express = require('express');
const { markdownlint } = require('markdownlint/promise');

const app = express();
app.use(express.json());

app.post('/api/lint', async (req, res) => {
  const { content } = req.body;
  
  try {
    const result = await markdownlint({
      strings: { 'input': content },
      config: { default: true }
    });
    
    res.json({
      errors: result.input || [],
      success: (result.input || []).length === 0
    });
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

app.listen(3000);
```

### Build Tool Integration

```javascript
// gulpfile.js
const gulp = require('gulp');
const { markdownlint } = require('markdownlint/promise');
const through2 = require('through2');

function lintMarkdown() {
  return gulp.src('**/*.md')
    .pipe(through2.obj(async (file, enc, callback) => {
      const result = await markdownlint({
        strings: {
          [file.path]: file.contents.toString()
        },
        config: { default: true }
      });
      
      const errors = result[file.path] || [];
      if (errors.length > 0) {
        console.error(`${file.path}:`);
        errors.forEach(err => {
          console.error(`  ${err.lineNumber}: ${err.ruleDescription}`);
        });
      }
      
      callback(null, file);
    }));
}

gulp.task('lint:md', lintMarkdown);
```

## Best Practices

1. **Use Promise API** - Cleaner async/await syntax
2. **Cache Config** - Load configuration once and reuse
3. **Batch Operations** - Lint multiple files in one call
4. **Handle Errors** - Always catch and handle errors properly
5. **Apply Fixes Carefully** - Test fixes before writing to disk
6. **Custom Rules** - Create project-specific validation rules
7. **Report Formatting** - Format results for readability

## Common Pitfalls

1. **Async/Sync Mismatch** - Don't mix callback, promise, and sync APIs
2. **Missing Config** - Always provide a config object
3. **Forgetting to Handle Errors** - Check result object for errors
4. **Not Testing Fixes** - Verify fixes don't break content
5. **Memory Issues** - Don't load all files at once for large projects
6. **Config Caching** - Don't reload config on every lint call
