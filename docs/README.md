# tauri程序示例

## 创建项目

1. **安装 Yarn:**
   ```bash
   npm install -g yarn
   ```
   你使用了 `npm` 全局安装了 Yarn，这是一个流行的 JavaScript 包管理器。相比 `npm`，Yarn 提供了更快的依赖安装和一致的依赖解析机制，适合项目的依赖管理。

2. **创建 Tauri 应用:**
   ```bash
   yarn create tauri-app
   ```
   这里你使用 Yarn 创建了一个 Tauri 应用。`yarn create` 命令会帮助你生成一个基本的 Tauri 项目结构，包含了 Tauri 的主进程、前端代码和相关配置文件。

   Tauri 是一个非常轻量的桌面应用框架，它的特点是可以用任意的前端框架（如 Vue、React 等）来开发应用，而不需要依赖 Electron 这样较大的运行时。

3. **进入项目目录并安装依赖:**
   ```bash
   cd <project-directory>
   yarn
   ```
   在进入到你的 Tauri 项目目录后，`yarn` 命令会自动读取 `package.json` 文件并安装所需的项目依赖。这个步骤下载了所有你项目所需的 JavaScript 包。

4. **启动开发服务器 (前端开发):**
   ```bash
   yarn dev
   ```
   这一步会启动前端开发服务器，通常这是基于 Vue.js 的开发模式，它会自动编译和热更新前端代码。你可以在浏览器中看到应用的前端部分。

5. **启动 Tauri 开发环境 (桌面应用开发):**
   ```bash
   yarn tauri dev
   ```
   这个命令启动了 Tauri 的开发模式，结合了桌面环境和前端开发。它会同时启动前端的开发服务器并在桌面环境中加载前端页面。你可以看到应用在桌面窗口中运行，而不是仅仅在浏览器中。

   **Tauri 关键部分**：
   - Tauri 的后端是一个 Rust 程序，它负责与操作系统的交互，比如文件系统访问、窗口管理等。
   - 你的 Vue.js 前端作为 UI 部分，利用 Tauri 提供的 API 与系统层交互。

### 总结
1. 通过 `npm install -g yarn` 安装 Yarn。
2. 使用 `yarn create tauri-app` 创建一个新的 Tauri 项目。
3. 进入项目目录并使用 `yarn` 安装依赖。
4. 使用 `yarn dev` 启动 Vue 前端开发服务器。
5. 使用 `yarn tauri dev` 启动 Tauri 的桌面应用开发环境。

这个流程已经帮你搭建了一个 Vue.js 和 Tauri 的桌面应用开发环境，你可以在此基础上进行功能开发，比如与操作系统交互、文件读取、窗口管理等。

## 解释一下

在 Tauri 中，前端（Vue.js）与后端（Rust）之间的通信是通过 Tauri 提供的 `invoke` 函数进行的。具体来说，`invoke` 函数允许前端通过 IPC（进程间通信）向 Tauri 后端的命令发出请求，然后后端处理请求并将结果返回给前端。

你定义的 `greet` 命令在 Rust 部分如下：

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

这个 `greet` 函数被标记为 `#[tauri::command]`，这使它成为可以通过 Tauri 的前端调用的命令。

### 如何从 Vue.js 调用这个 `greet` 函数

在 Vue.js 中，你需要使用 Tauri 的 `@tauri-apps/api` 库来调用 `greet` 函数。通常你会在 `script` 部分通过 `invoke` 函数进行调用，类似于这样：

```ts
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      name: '',
      greeting: ''
    };
  },
  methods: {
    async greet() {
      this.greeting = await invoke('greet', { name: this.name });
    }
  }
};
```

在这个例子中：
- `invoke('greet', { name: this.name })`：这是调用 Rust 端的 `greet` 命令，其中 `name` 是你传递给命令的参数。
- `this.greeting = await invoke(...)`：Rust 返回的结果会被存储在 `this.greeting` 中，前端可以使用它来展示结果。

### 前端示例：
```html
<template>
  <div>
    <input v-model="name" placeholder="Enter your name" />
    <button @click="greet">Greet</button>
    <p>{{ greeting }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const name = ref('');
const greeting = ref('');

async function greet() {
  greeting.value = await invoke('greet', { name: name.value });
}
</script>
```

在这个 Vue.js 前端代码中，当用户输入名字并点击按钮时，`greet` 方法会调用 Rust 后端的 `greet` 命令，并将返回的问候语显示在页面上。

### 总结
- Tauri 的 `invoke` 函数让前端能够通过命令与 Rust 后端交互。
- Rust 中的 `#[tauri::command]` 标记使得 `greet` 函数可以被前端调用。
- Vue.js 通过 `invoke('greet', { name })` 调用 Rust 端的命令并接收返回值。

这样就实现了前端和后端的通信！

## 如何再增加更多的rust命令可被前端调用

要增加更多的 Rust 命令供前端调用，你只需要按照以下步骤扩展 Rust 代码，并确保在 `tauri::Builder` 中注册新的命令。具体步骤如下：

### 1. 定义新的命令
首先，你可以在 Rust 中定义更多的命令，类似于你现有的 `greet` 命令。每个命令都需要用 `#[tauri::command]` 标记。

例如，增加一个新的 `calculate_sum` 命令来计算两个数字的和：

```rust
#[tauri::command]
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
```

### 2. 注册新命令
接下来，在 `main` 函数中注册新命令。你需要使用 `tauri::generate_handler!` 注册所有要暴露给前端调用的命令。

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, calculate_sum]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. 在前端调用新的命令
在 Vue.js 前端，你可以使用与 `greet` 相同的方式，通过 `invoke` 调用新的命令。

例如，调用 `calculate_sum` 命令：

```html
<template>
  <div>
    <input v-model.number="num1" type="number" placeholder="Enter number 1" />
    <input v-model.number="num2" type="number" placeholder="Enter number 2" />
    <button @click="calculate">Calculate Sum</button>
    <p>Sum: {{ sum }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const num1 = ref(0);
const num2 = ref(0);
const sum = ref(0);

async function calculate() {
  sum.value = await invoke('calculate_sum', { a: num1.value, b: num2.value });
}
</script>
```

在这个例子中：
- 用户输入两个数字，然后点击按钮触发 `calculate` 函数。
- `calculate` 函数通过 `invoke('calculate_sum', { a: num1.value, b: num2.value })` 调用 Rust 后端的 `calculate_sum` 命令。
- 结果被保存到 `sum` 中，并显示在页面上。

### 4. 多个命令的综合应用
你可以在 Rust 中定义任意数量的命令，并通过 `tauri::generate_handler!` 将它们注册到 Tauri 应用中。这种方法使得前端可以灵活地调用后端逻辑，实现复杂的功能。

### 总结
- **定义命令：** 在 Rust 代码中定义新函数，并使用 `#[tauri::command]` 标记它们。
- **注册命令：** 在 `tauri::Builder` 中使用 `tauri::generate_handler!` 注册新命令。
- **前端调用：** 在前端通过 `invoke('命令名称', 参数)` 调用这些命令，并处理返回结果。

这样你可以非常灵活地增加任意数量的 Rust 后端功能供前端调用。

## 创建Julia后端

