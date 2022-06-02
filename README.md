# Nuke
A universal Node.js version manager for Windows.

## Features
- Download and install Node.js
- Switching versions by command/environment variables
- Use different versions at the same time

## Why not nvm-for-windows?
Nuke and nvm-for-windows take different technical routes. nvm-for-windows manages Node.js versions by creating symbol link under `C:\Program Files`, which means that only one version of Node.js can be used at the same time. Nuke uses a principle similar to Cargo of Rust, which creates launchers with the same name as the Node.js commands. Launcher detects the version of Node.js the user wants to use when running, and then launches the corresponding Node.js. This solution allows users to use any version of Node.js at the same time.

## Setup
### I. Install Nuke
1. Download nuke from here.
2. Launch `nuke.exe`
3. Press `Y` to allow nuke to install on your computer.
4. Open a command prompt (e.g. cmd, powershell) and try the `nuke` command.

### II. Install Node.js
Nuke provides the `install` command for installing Node.js.
```
nuke install <Node.js Version> [--arch <Arch>]
```
For example, if you want to install the latest version of Node.js with major version `16`, you can use the following command：
```
nuke install 16
```
### III. Switch Node.js Version
When you have multiple versions of Node.js installed, you can switch versions in two ways.
#### Command
The `nuke default` command allows you to switch the version of Node.js used globally.
```
nuke default 16
```
#### Environment Variables
By setting the `NUKE_NODE_VERSION` and `NUKE_NODE_ARCH` environment variables, you can specify the version of Node.js used by the current shell session, which does not effect the other environments.
```powershell
$env:NUKE_NODE_VERSION=16
$env:NUKE_NODE_ARCH=x86

npm install
```
#### IIII. See more help
```
nuke -h
```