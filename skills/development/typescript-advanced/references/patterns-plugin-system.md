# Plugin System Architecture

Build extensible applications with type-safe plugin systems.

## Basic Plugin Interface

```typescript
interface Plugin {
  readonly name: string;
  readonly version: string;
  
  init(): Promise<void>;
  cleanup(): Promise<void>;
}

interface PluginManager {
  register(plugin: Plugin): void;
  unregister(name: string): Promise<void>;
  get(name: string): Plugin | undefined;
  list(): Plugin[];
}
```

## Typed Plugin with Configuration

```typescript
interface PluginConfig<T = unknown> {
  name: string;
  version: string;
  config: T;
}

interface Plugin<TConfig = unknown> {
  readonly name: string;
  readonly version: string;
  
  init(config: TConfig): Promise<void>;
  cleanup(): Promise<void>;
}

// Example: Logger plugin
interface LoggerConfig {
  level: 'debug' | 'info' | 'warn' | 'error';
  output: 'console' | 'file';
  filePath?: string;
}

class LoggerPlugin implements Plugin<LoggerConfig> {
  readonly name = 'logger';
  readonly version = '1.0.0';
  
  private config?: LoggerConfig;
  
  async init(config: LoggerConfig) {
    this.config = config;
    console.log(`Logger initialized: ${config.level} -> ${config.output}`);
  }
  
  async cleanup() {
    console.log('Logger cleaned up');
  }
  
  log(level: LoggerConfig['level'], message: string) {
    if (!this.config) {
      throw new Error('Plugin not initialized');
    }
    // Logging logic
  }
}
```

## Plugin with Hooks

```typescript
type PluginHook<TData = unknown> = (data: TData) => TData | Promise<TData>;

interface HookablePlugin<THooks extends Record<string, PluginHook>> {
  readonly name: string;
  hooks: THooks;
}

// Example: Transform plugin
interface TransformHooks {
  beforeTransform: PluginHook<string>;
  afterTransform: PluginHook<string>;
}

class TransformPlugin implements HookablePlugin<TransformHooks> {
  readonly name = 'transform';
  
  hooks: TransformHooks = {
    beforeTransform: (data) => data.trim(),
    afterTransform: (data) => data.toUpperCase()
  };
  
  async transform(input: string): Promise<string> {
    let result = await this.hooks.beforeTransform(input);
    // Main transformation
    result = result.replace(/foo/g, 'bar');
    result = await this.hooks.afterTransform(result);
    return result;
  }
}
```

## Plugin Registry with Type Safety

```typescript
interface PluginRegistry {
  plugins: Map<string, Plugin>;
}

class TypedPluginRegistry implements PluginRegistry {
  plugins = new Map<string, Plugin>();
  
  register<T extends Plugin>(plugin: T): void {
    if (this.plugins.has(plugin.name)) {
      throw new Error(`Plugin ${plugin.name} already registered`);
    }
    this.plugins.set(plugin.name, plugin);
  }
  
  get<T extends Plugin>(name: string): T | undefined {
    return this.plugins.get(name) as T | undefined;
  }
  
  async unregister(name: string): Promise<void> {
    const plugin = this.plugins.get(name);
    if (plugin) {
      await plugin.cleanup();
      this.plugins.delete(name);
    }
  }
  
  list(): Plugin[] {
    return Array.from(this.plugins.values());
  }
}
```

## Plugin Lifecycle Management

```typescript
type PluginState = 'registered' | 'initializing' | 'active' | 'error' | 'disabled';

interface ManagedPlugin<TConfig = unknown> extends Plugin<TConfig> {
  state: PluginState;
  error?: Error;
}

class PluginManager {
  private plugins = new Map<string, ManagedPlugin>();
  
  async register<TConfig>(
    plugin: Plugin<TConfig>,
    config: TConfig
  ): Promise<void> {
    const managed: ManagedPlugin<TConfig> = {
      ...plugin,
      state: 'registered'
    };
    
    this.plugins.set(plugin.name, managed);
    
    try {
      managed.state = 'initializing';
      await plugin.init(config);
      managed.state = 'active';
    } catch (error) {
      managed.state = 'error';
      managed.error = error as Error;
      throw error;
    }
  }
  
  getState(name: string): PluginState | undefined {
    return this.plugins.get(name)?.state;
  }
}
```

## Plugin Dependencies

```typescript
interface PluginWithDeps<TConfig = unknown> extends Plugin<TConfig> {
  dependencies: string[];
}

class DependencyAwareManager {
  private plugins = new Map<string, PluginWithDeps>();
  
  async register<TConfig>(
    plugin: PluginWithDeps<TConfig>,
    config: TConfig
  ): Promise<void> {
    // Check dependencies
    for (const dep of plugin.dependencies) {
      if (!this.plugins.has(dep)) {
        throw new Error(
          `Missing dependency: ${dep} required by ${plugin.name}`
        );
      }
    }
    
    this.plugins.set(plugin.name, plugin);
    await plugin.init(config);
  }
  
  // Topological sort for initialization order
  private getInitOrder(): string[] {
    // Implementation of topological sort
    return [];
  }
}
```

## Best Practices

1. **Define clear interfaces**: Separate plugin contract from implementation
2. **Version plugins**: Track compatibility
3. **Manage lifecycle**: Init, active, cleanup states
4. **Handle dependencies**: Ensure plugins load in correct order
5. **Type configurations**: Each plugin type has specific config
6. **Error handling**: Gracefully handle plugin failures
7. **Isolation**: Plugins shouldn't access each other directly
