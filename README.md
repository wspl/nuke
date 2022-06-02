# Nuke
A Node.js version manager for Windows, aimed at using different versions of Node.js in different terminals at the same time.
## Features
- Download and manage multiple Node.js versions (and architectures)
- Switch the version used by the current shell via environment variables
- Switch the global(or default) version via command
- No version conflicts: terminals using different versions do not interfere with each other.

## Why not nvm-for-windows?
Nuke uses a different solution than nvm-for0windows. nvm-for-windows manages Node.js versions by creating shell link (or symbol link) under `%PROGRAMFILES%`, which means that only one version of Node.js can be used at the same time. Nuke uses a pratice similar to `cargo` of Rust, which creates launchers with the same name as the Node.js commands. Launcher detects the version of Node.js the user wants to use when running, and then launches the corresponding Node.js. This approach allows users to use any version of Node.js at the same time.

## Setup
### I. Install Nuke
1. Download nuke from [GitHub Releases](https://github.com/wspl/nuke/releases).
2. Double click to start `nuke.exe`
3. Press `Y` to allow nuke to install on your computer.
4. Open a command prompt (e.g. cmd, powershell) and use the `nuke` command.

### II. Install Node.js
Nuke provides the `install` command for installing Node.js.
```
nuke install <Node.js Version> [--arch <Arch>]
```
For example, if you want to install the latest version of Node.js with major version `18`, you can use the following command：
```
nuke install 18
```
### III. Switch Node.js Version
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
### IIII. More help
```
nuke -h
```