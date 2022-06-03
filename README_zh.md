# Nuke

[![Release](https://img.shields.io/github/v/release/wspl/nuke?include_prereleases)](https://github.com/wspl/nuke/releases)
[![Build](https://github.com/wspl/nuke/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/wspl/nuke/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/wspl/nuke)](https://github.com/wspl/nuke/blob/master/LICENSE)

[English README](./README.md)

这是一个开箱即用的 Node.js 版本管理器，用于 Windows 系统。目标在于在不同的终端会话中同时使用不同的 Node.js 版本。

**本工具还缺少许多重要的功能，但是最终把这些功能都实现。当然也欢迎大家的 PR。**

## 功能
- 下载和管理多个 Node.js 版本（与架构）
- 通过环境变量来切换当前终端所使用的 Node.js 版本
- 通过命令来切换全局默认 Node.js 版本
- 没有版本冲突：多个终端使用不同版本的 Node.js 不会干扰其他终端

## 对比
|                                   | Nuke | nvm-windows | fnm  | nodist  |
|-----------------------------------|------|-------------|------|---------|
| 开发语言                          | Rust | Go          | Rust | Node.js |
| 零配置                | ✅  | ✅          |      |         |
| 同时使用多个版本 | ✅  |             | ✅   | ✅     |
| 无需 Post Script           | ✅  | ✅          |      | ✅     |
| 不需要管理员权限   | ✅  |             | ✅   |         |
| 安装不同的 Node.js 架构        | ✅  | ✅          | ✅   |         |

目前我对这些工具的调查并不深入，因此如果有错误的话欢迎纠正。

## 安装和使用
### 1. 安装 Nuke
1. 从 [GitHub Releases](https://github.com/wspl/nuke/releases) 下载 Nuke。
2. 双击并启动 `nuke.exe`。
3. 输入 `Y` 以允许 Nuke 在您的计算机上安装。
4. 启动一个终端（例如：cmd, powershell），并使用 `nuke` 命令。

### 2. 安装 Node.js
Nuke 提供 `install` 命令用于安装 Node.js。
```
nuke install <Node.js 版本> [--arch <架构>]
```
例如，如果你想要安装最新版本的 Node.js 18，你可以使用如下命令：
```
nuke install 18
```
### 3. 切换 Node.js 版本
当你已经安装了多个 Node.js 版本，你可以通过两种方式切换版本：
#### 命令
`nuke default` 命令允许你切换全局默认使用的 Node.js 版本。
```
nuke default 18
```
#### 环境变量
通过设置 `NUKE_NODE_VERSION` 和 `NUKE_NODE_ARCH` 环境变量，你可以指定当前终端环境所使用的 Node.js 版本，并且不会影响到其他的终端环境。
```powershell
$env:NUKE_NODE_VERSION=18
$env:NUKE_NODE_ARCH=x86

npm install
```
### 4. 更多帮助
```
nuke -h
```

## 工作原理
当 Nuke 被安装后，它会在 `bin` 目录下创建几个可执行文件 `node.exe`、`npm.exe` 和 `npx.exe`，这些文件本质上是 Nuke Launcher。`bin` 目录会在 Nuke 安装的时候被注册到 PATH 环境变量中。当你在使用 Node.js 之前，你可以在环境变量中设置需要使用的 Node.js 版本，这样一来，通过 `node` 命令启动的 Nuke Launcher 就能借此将 Node.js 作为子进程启动，并透传所有参数。