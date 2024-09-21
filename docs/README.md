# Tauri 程序示例

## 创建项目

1. **安装 Yarn:**

   ```bash
   npm install -g yarn
   ```

   使用 `npm` 全局安装 Yarn，这是一个流行的 JavaScript 包管理器，提供比 `npm` 更快、更一致的依赖安装方式，适合项目的依赖管理。

2. **创建 Tauri 应用:**

   ```bash
   yarn create tauri-app
   ```

   通过 Yarn 创建一个 Tauri 应用。`yarn create` 命令会生成一个基本的 Tauri 项目结构，包含 Tauri 的主进程、前端代码和相关配置文件。

   **Tauri** 是一个非常轻量的桌面应用开发框架，可以结合 Vue、React 等前端框架构建应用，无需依赖较大的运行时如 Electron。

3. **进入项目目录并安装依赖:**

   ```bash
   cd <project-directory>
   yarn
   ```

   进入你的 Tauri 项目目录后，运行 `yarn` 安装项目所需的所有依赖。

4. **启动开发服务器（前端开发）：**

   ```bash
   yarn dev
   ```

   这将启动前端开发服务器，通常基于 Vue.js 的开发模式，能够自动编译并热更新前端代码。你可以在浏览器中查看前端页面。

5. **启动 Tauri 开发环境（桌面应用开发）：**

   ```bash
   yarn tauri dev
   ```

   该命令会启动 Tauri 的开发环境，结合桌面和前端的开发体验。你可以在桌面窗口中看到应用运行，而不仅仅是在浏览器中。

   **Tauri 关键部分**：
   - Tauri 后端是一个 Rust 程序，负责与操作系统交互（如文件系统、窗口管理等）。
   - Vue.js 前端作为 UI，通过 Tauri 提供的 API 与系统层交互。

### 总结

1. 使用 `npm install -g yarn` 安装 Yarn。
2. 通过 `yarn create tauri-app` 创建 Tauri 项目。
3. 进入项目目录，使用 `yarn` 安装依赖。
4. 运行 `yarn dev` 启动 Vue 前端开发服务器。
5. 运行 `yarn tauri dev` 启动 Tauri 桌面应用开发环境。

---

## 创建 Julia 后端

### 1. 进入 `src-julia` 目录并创建 Julia 项目

在项目根目录下创建 `src-julia` 文件夹，存放 Julia 后端代码。

### 2. 创建 `julia-server.jl` 文件并编写后端代码

在 `src-julia` 中创建 `julia-server.jl` 文件，并添加以下内容：

```julia
using Oxygen
using HTTP
using JSON3

# 定义主路由函数，返回帮助信息
function main_help(req::HTTP.Request)
    return "使用 /ping 检查 API 的健康状态。"
end

# 定义健康检查路由函数
function health_check(req::HTTP.Request)
    return Dict("ai4ejuliaapi" => "healthy!")
end

# 定义处理 `/add/{x}/{y}` 路由的函数
function add(req::HTTP.Request, x, y)
    x_parsed = parse(Float64, x)
    y_parsed = parse(Float64, y)
    return string(x_parsed + y_parsed)  # 返回计算结果作为字符串
end

# 定义处理 POST 请求的函数
function add_post(req::HTTP.Request)
    # 解析 JSON 请求体
    data = JSON3.read(String(req.body))
    x = parse(Float64, data["x"])
    y = parse(Float64, data["y"])
    return string(x + y)  # 返回计算结果作为字符串
end

# 初始化主路由
function InitMainRouter()
    Oxygen.route([Oxygen.GET], "/", main_help)
    Oxygen.route([Oxygen.GET], "/ping", health_check)
    Oxygen.route([Oxygen.GET], "/add/{x}/{y}", add)
    Oxygen.route([Oxygen.POST], "/add_post", add_post)
end

# 初始化路由
function InitRouter()
    println("正在初始化主路由")
    InitMainRouter()
end

# 主函数，启动服务器
function julia_main()::Cint
    InitRouter()
    Oxygen.serve(host="0.0.0.0", port=19801, show_banner=false)
    return 0
end

# 启动服务器
julia_main()
```

### 3. 激活 Julia 环境并安装所需包

#### 激活 Julia 环境

在 `src-julia` 目录下启动 Julia：

```bash
julia
```

进入 Julia REPL 后，使用 `Pkg` 模式（按 `]` 进入）来创建并激活项目环境：

```julia
] activate .
```

#### 安装所需的包

在激活环境后，使用以下命令安装所需的依赖包：

```julia
] add Oxygen HTTP JSON3
```

#### 验证包是否安装成功

安装成功后，回到 Julia 的普通模式，运行以下代码，检查包是否安装成功：

```julia
using Oxygen
using HTTP
using JSON3
```

如果没有报错，说明安装成功。

#### 保存环境信息

Julia 会在当前目录生成 `Project.toml` 和 `Manifest.toml` 文件。确保将这些文件保存到版本控制中，以便其他开发者能够通过 `instantiate` 命令重现相同的开发环境。

#### 将来如何重新激活环境

每次重新进入该项目目录时，可以使用以下命令重新激活环境并安装依赖：

```bash
julia
] activate .
] instantiate
```

---

## 启动 Julia 后端服务

在 `src-julia` 目录下，使用以下命令启动 Julia 服务器：

```bash
julia --project=. julia-server.jl
```

---

## 测试请求

你可以使用 `curl` 测试后端的 GET 和 POST 请求，以下是示例：

1. **GET 请求测试：**

   ```bash
   curl http://localhost:19801/add/3/5
   ```

   预期输出：`8.0`

2. **POST 请求测试：**

   发送包含字符串类型 JSON 的 POST 请求：

   ```bash
   curl -X POST http://localhost:19801/add_post -H "Content-Type: application/json" -d '{"x": "3.5", "y": "4.5"}'
   ```

   预期输出：`8.0`

---

通过这些步骤，你可以搭建一个基于 Tauri 和 Julia 的桌面应用程序，前端使用 Vue.js，后端通过 Julia 提供服务。

改写后的内容如下：

---

## 扩展计划

我们可以进一步扩展架构，基于 Tauri、Go 和 Julia 构建一个更加模块化、可扩展的应用系统：

1. **建立 Go 服务层**：
   可以在项目目录中创建一个 `src-go` 文件夹，将 Go 服务的代码放入其中。Go 服务将作为后端引擎，负责处理前端与后端的逻辑交互、用户鉴权和管理。

   **Go 的定位**：
   - Go 负责处理前端的 HTTP 请求，提供前端用户的认证和鉴权功能。
   - Go 作为服务引擎，处理业务逻辑，例如数据验证、持久化等功能。
   - Go 也可以作为代理调用 Julia，Go 负责与 Julia 交互，管理计算任务的调度。

2. **启动 Go 进程**：
   在 Rust（Tauri 主进程）中启动 Go 进程。通过 Rust 的 `Command` 模块启动 Go 服务进程，并通过 Rust 与 Go 进行通信。

3. **Go 调用 Julia**：
   Go 服务可以通过 HTTP 请求调用 Julia 后端计算服务，将计算任务委派给 Julia。这样，Julia 只需要专注于处理高性能计算任务，而不需要处理用户身份验证和授权等问题。

   **Julia 的定位**：
   - Julia 作为计算引擎，专注于数学运算、科学计算等高性能任务。
   - Julia 不再处理前端请求，而是由 Go 服务来转发处理好的计算任务。

4. **更好的架构分层**：
   - **Rust（Tauri 前端进程）**：负责启动前端桌面应用，控制 Go 和 Julia 进程的启动与停止，并通过前端与后端进行交互。
   - **Go（服务引擎）**：处理前端请求、业务逻辑和用户管理，调用 Julia 进行计算。
   - **Julia（计算引擎）**：仅负责高性能计算任务，作为后端的后端处理实际的计算。

---

## 使用 Wails 构建

除了使用 Tauri 之外，另一种选择是基于 **Wails** 来构建这类桌面应用程序。Wails 是一个轻量的 Go 桌面应用框架，结合了 Go 后端与前端框架（如 Vue.js、Svelte 等）。Wails 提供了类似 Tauri 的功能，但基于 Go 的主进程。

1. **Go 主进程**：通过 Wails，Go 直接成为主进程，前端可以使用 Vue、Svelte 等框架构建用户界面。
2. **Go + Julia 架构**：与 Tauri 相似，Go 可以管理后端服务和前端通信，而 Julia 继续作为计算引擎提供高效计算。

### Wails 的优势：
- 基于 Go，更容易与现有的 Go 服务集成。
- 提供简单的 API 用于创建跨平台桌面应用。
- 结合 Go 和前端框架（Vue、React 等），减少依赖其他语言（如 Rust）。

### 选择 Tauri 和 Wails 的对比：
- **Tauri** 更轻量，Rust 提供了更强的性能和系统级控制。
- **Wails** 结合了 Go 的简洁和强大的并发处理能力，适合那些已经在使用 Go 的团队。

---

### 总结

通过将 Go 和 Julia 结合，Go 负责服务引擎的部分，而 Julia 专注于计算引擎，我们可以打造一个高效、清晰的分层架构。前端与后端解耦，允许 Julia 进行高效计算，同时 Go 负责用户管理与任务分配。通过 Tauri 或 Wails，我们能够根据不同需求选择最适合的桌面应用开发框架。