# Nuke

[![Release](https://img.shields.io/github/v/release/wspl/nuke?include_prereleases)](https://github.com/wspl/nuke/releases)
[![Build](https://github.com/wspl/nuke/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/wspl/nuke/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/wspl/nuke)](https://github.com/wspl/nuke/blob/master/LICENSE)

[中文介绍](./README_zh.md)

An out-of-the-box Node.js version manager for Windows, aimed at using different versions of Node.js in different terminals at the same time.

**There are still some key features missing from this tool, and I will eventually complete them. If you want to participate, please feel free to create a PR.**

## Features
- Download and manage multiple Node.js versions (and architectures)
- Switch the version used by the current shell via environment variables
- Switch the global(or default) version via command
- No version conflicts: terminals using different versions do not interfere with each other

## Comparison
|                                   | Nuke | nvm-windows | fnm  | nodist  |
|-----------------------------------|------|-------------|------|---------|
| Language                          | Rust | Go          | Rust | Node.js |
| Zero configuration                | ✅  | ✅          |      |         |
| Use multiple version at same time | ✅  |             | ✅   | ✅     |
| No post-script required           | ✅  | ✅          |      | ✅     |
| No elevated privileges required   | ✅  |             | ✅   |         |
| Install different Node.js architectures        | ✅  | ✅          | ✅   |         |

My investigation may not have gone far enough, please feel free to correct any inaccurate information.

## Setup
### 1. Install Nuke
1. Download nuke from [GitHub Releases](https://github.com/wspl/nuke/releases).
2. Double click to start `nuke.exe`
3. Press `Y` to allow nuke to install on your computer.
4. Open a command prompt (e.g. cmd, powershell) and use the `nuke` command.

### 2. Install Node.js
Nuke provides the `install` command for installing Node.js.
```
nuke install <Node.js Version> [--arch <Arch>]
```
For example, if you want to install the latest version of Node.js with major version `18`, you can use the following command：
```
nuke install 18
```
### 3. Switch Node.js Version
When you have multiple versions of Node.js installed, you can switch versions in two ways.
#### Command
The `nuke default` command allows you to switch the version of Node.js used globally.
```
nuke default 18
```
#### Environment Variables
By setting the `NUKE_NODE_VERSION` and `NUKE_NODE_ARCH` environment variables, you can specify the version of Node.js used by the current shell session, which does not effect the other environments.
```powershell
$env:NUKE_NODE_VERSION=18
$env:NUKE_NODE_ARCH=x86

npm install
```
### 4. More help
```
nuke -h
```

## Why a new Node.js version manager
I often need to use both 64-bit and 32-bit Node.js for some build tasks on a single windows continuous integration machine. At the same time there are other guys using Node.js on this server to do other things. But nvm-windows doesn't allow us to use different versions of Node.js at the same time. When I switched to other alternative tools, the complex configuration gave me a huge headache. So I simply wrote my own tool to solve this problem.

## How does it work?
Once Nuke is installed, it creates several executables in the `bin` folder named `node.exe`, `npm.exe` and `npx.exe`, which are actually Nuke Launcher. The `bin` folder is added to the operating system's PATH environment variable when Nuke is installed. Before you need to use Node.js, you can set an environment variable that lets Nuke Launcher know what version of Node.js you want to use, so that when you start Nuke Launcher with the `node` command, it knows what version of Node.js to launch and starts it as a child process with all parameters passed through.