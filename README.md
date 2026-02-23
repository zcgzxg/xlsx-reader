# xlsx-reader

`xlsx-reader` 是一个基于 Rust 与 WebAssembly 的 Excel（`.xlsx`）读取库，适合在浏览器环境中解析文件并返回结构化数据。

## 项目作用

- 接收 `.xlsx` 文件的二进制数据（`Uint8Array` / `ArrayBuffer`）。
- 解析工作簿中的所有工作表。
- 为每个工作表输出：
  - `headers`：首行表头（可能为空）。
  - `rows`：从第二行开始的二维数据数组。
- 输出结果按“工作表名 -> 数据”组织，便于前端直接消费。

## 返回数据结构

函数返回值在 TypeScript 中可理解为：

```ts
Map<
  string,
  {
    headers: string[] | null;
    rows: (string | number | boolean | undefined)[][];
  }
>
```

## 使用方式

### 1. 环境准备

需要准备以下工具：

- Rust Nightly（建议使用 `rustup` 安装）
- `wasm-bindgen-cli`
- `wasm-opt`（用于压缩 WASM 体积，来自 `binaryen`）
- `static-compress`
- Node.js（用于运行 `web/` 下的 Vue 示例）
- `ast-grep`（仅微信小程序构建需要）

安装示例：

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-bindgen-cli static-compress
# wasm-opt 可通过系统包管理器安装，如 npm install -g binaryen、brew install binaryen 或 apt install binaryen
```

`ast-grep` 可参考官方安装方式（如 npm、cargo 或二进制包）安装，并确保命令 `sg` 可用。

### 2. 构建浏览器版本（wasm-bindgen）

在项目根目录执行：

```bash
./build.sh
```

该脚本会完成：

- `cargo +nightly build --release`
- 使用 `wasm-opt` 进行深度体积优化
- 使用 `wasm-bindgen` 生成 JS/WASM 绑定文件到 `web/src/xlsx-reader`
- 对 `.wasm` 文件做 brotli 压缩

### 3. 在浏览器中查看（Vue 示例）

本项目通过 `./web` 下的 Vue 项目进行浏览器侧验证与查看。

```bash
cd web
pnpm install
pnpm dev
```

启动后按 Vite 输出地址在浏览器打开即可。

### 4. 构建微信小程序版本

在项目根目录执行：

```bash
./build-mp.sh
```

该脚本会输出到 `dist-mp/`，并调用 `sg scan -U ./dist-mp` 按项目规则自动改写生成代码以兼容小程序环境，因此必须先安装 `ast-grep`。

## 开发说明

- 本项目核心解析依赖：`calamine`。
- 序列化与 JS 互操作依赖：`serde`、`serde-wasm-bindgen`、`wasm-bindgen`。
- 默认启用 `console_error_panic_hook`，便于在浏览器控制台查看 panic 信息。
