---
title: 更新笔记
description: JLiverTool 的版本记录与功能变化。
---

All notable changes to this project will be documented in this file.

## [3.0.4] - 2026-02-11

### 🚀 Features

- Click through

## [3.0.3] - 2026-02-10

### 🚀 Features

- Danmu item wrap

### ⚙️ Miscellaneous Tasks

- Decrease the minimal height of main window

## [3.0.2] - 2026-02-08

### 🚀 Features

- Cutoff and warning message

### 📚 Documentation

- Update

## [3.0.1] - 2026-02-08

### 🐛 Bug Fixes

- Room title edit

### 📚 Documentation

- Update

## [3.0.0] - 2026-02-07

### 🚀 Features

- Refactor rust (#47)

### ⚙️ Miscellaneous Tasks

- Update package

## [2.4.4] - 2026-01-12

### 🐛 Bug Fixes

- Add dynamic emoji map

### 📚 Documentation

- Update

## [2.4.3] - 2025-09-01

### 🚀 Features

- Copy rtmp info

### 🐛 Bug Fixes

- Start/end live

## [2.4.2] - 2025-06-27

### 🐛 Bug Fixes

- Code -352 for GetDanmuInfo api

## [2.4.1] - 2025-05-26

### 🐛 Bug Fixes

- Add wbi sign for danmu api

## [2.4.0] - 2025-05-25

### 🚀 Features

- Optimize with requestAnimationFrame
- Support aliyun tts
- Support tts custom endpoint
- Support aliyun tts sdk

## [2.3.0] - 2025-04-23

### 🚀 Features

- Add landing page
- Basic plugin manager
- Implement example plugin
- Plugin management
- More events for plugin
- Fix stall when cookies expire

### 🐛 Bug Fixes

- Plugin list update
- Wrong guard entry message

### 📚 Documentation

- Update
- Update
- Update

### ⚙️ Miscellaneous Tasks

- Update dependencies
- Deploy pages

## [2.2.1] - 2024-12-21

### 🚀 Features

- Extend send input when focus
- Delay backend_service avoiding blank window when checking updates
- Change mainwindow watcher count data source

### 🐛 Bug Fixes

- Disable danmu-input's spellcheck and scrollbar
- Author info

### ⚙️ Miscellaneous Tasks

- Adjust window loaded callback log level

## [2.2.0] - 2024-08-20

### 🚀 Features

- Tts support

### 🐛 Bug Fixes

- Replace afdian url
- Always-on-top config
- Create tts one by one
- Interact message style

### 💼 Other

- Upload to oss
- Add more url source for download

### ⚙️ Miscellaneous Tasks

- Add helper for updating release page
- Update release script
- Update
- Update

## [2.1.3] - 2024-06-04

### 🐛 Bug Fixes

- Sc window clear button
- Ws reconnect
- Msg handle

### 💼 Other

- Remove oss upload
- Create draft for manual confirm before release
- Using tag as version

### ⚙️ Miscellaneous Tasks

- Fixed dev version

## [2.1.2] - 2024-05-22

### 🐛 Bug Fixes

- Ws reconnect after manual disconnect
- Merge switch button not working

### 🚜 Refactor

- Message handlers

## [2.1.1] - 2024-01-28

### 💼 Other

- Fix privilege_type for non-guard entry (close #28)
- Add necessary retry for handling fetch exception when init (close #27)
- Fix ws reconnect (close #27)

## [2.1.0] - 2024-01-25

### 🚀 Features

- Differ enter effects from normal interact msg

### 💼 Other

- Add window to display rank list
- Fix replicated superchat caused by translation sc
- Fix border style
- Simplify css margin

### ⚙️ Miscellaneous Tasks

- Bump version to 2.1.0

## [2.0.0] - 2024-01-14

### 🚀 Features

- Provide support for linux version
- Upload artifacts of linux builds
- Prevent minimize when always on top

### 🐛 Bug Fixes

- App icon on linux build
- Types path
- Remove warning of setting-window.ts
- Fix all vulnerabilities

### 💼 Other

- Introduce logger
- File output added
- Refactor
- Update license copyright years
- Adjust style and add handle for invokes
- Add log display
- Remove extra close tag
- Get latest version from GitHub
- Enable window frame when develop
- Fix window under MacOS dev
- Fix medal border color
- Window click through settings
- Hot config of room id
- Fix room config load
- Add support for merge rooms
- Init merged rooms when setup
- Merge danmu render
- Preload and cache for font list
- Add theme selector
- Add theme config
- Decode title & fix font size
- Apply font config
- Font & font_size config for all window
- Fix window hide
- Add user info for merged danmus
- Trim and remove line break for content
- Add danmu cache
- Update room info after main window loaded
- Add detail window for danmu history
- Configs for detail window
- Fix scroll
- Add rate display
- Disable detail window when not login
- Reconnect after login
- Adjust medal font size
- Add handle for gift message
- Add handle for guard message
- Add clear gifts support
- Add handle for superchat message
- Add more room event support
- Adjust theme css
- Fix backend button & dropdown css
- Add support for interact message
- Record all action for detail & fix unlighted medal
- Hide/show interact message
- Ignore generated scaled danmu
- Adjust detail entry style & remove entry when disable interact
- Fix some lint issues
- Sort initial data
- Add themes & fix medal level color
- Remove unused methods
- Check update
- Add startup update check
- Add sponsor info
- Add sponsor goal progress bar
- Fix gift background for each theme
- Clear and reinit when room change
- Filename differ from v1
- Fix config set path
- Reverse record order & adjust  style
- Fix room title
- Fix for side conn
- Feat update room title
- Limit rate
- Update owned when room change
- Fix bv link
- Feat command invoke & optimize ignore_free
- Only show price of gold gifts
- Set ontop when init
- Add support for @user in danmu
- Add start/stop live
- Update
- Add advanced setting tab
- Add entry limit for main and detail
- Adjust lite-mode css
- Adjust lite-mode style
- Lite-mode for all windows
- Adjust min-width to 200px
- Fix dmg-license

### 🚜 Refactor

- Add more type info & organize files
- Refactor bili api with unit test
- Websocket and message [en/de]code
- Backend_service
- Add config_store
- JliverAPI in web content
- Rename main-window files
- Replace electron-store
- Msg handle
- Danmu msg parse
- New way to handle toggles
- Main window menu
- Ajust file name
- Danmu msg handle & render
- Medal render with info
- Extract svgs into files
- Adjust style and animation
- Basic setting page update

### 🎨 Styling

- Introduce WinUI3 style design

### 🧪 Testing

- Add 🤔 in test message

### ⚙️ Miscellaneous Tasks

- Ignore package-lock.json
- Format common.css
- Adjust file structure
- Add build option for single platform
- Add package-lock.json

## [1.4.1] - 2023-12-11

### 🐛 Bug Fixes

- Fit qrlogin api change

## [1.3.1] - 2023-03-07

### 🚀 Features

- Disable prompt while request version failed

### 🐛 Bug Fixes

- 平滑滚动可能不到底部
- MacOS下托盘图标大小的问题

### 📚 Documentation

- Update README.md
- Update README.md

## [1.3.0] - 2023-02-28

### 🐛 Bug Fixes

- Change github action runner

## [1.2.8] - 2022-09-30

### 🚀 Features

- 新增新版本检测

## [1.2.7] - 2022-09-30

### 🐛 Bug Fixes

- Multiple superchat display
- Remove getUserInfo & fix gift delete
- Msg GUARD_BUY

## [1.2.6] - 2022-09-19

### 🚀 Features

- 舰长信息在首次获取时保存，避免preload时重复查询

## [1.2.5] - 2022-09-19

### 🐛 Bug Fixes

- 修复了CMD变化可能导致的舰长不显示
- 去除上舰用户信息获取，接口现无法匿名使用
- 用户信息获取接口

## [1.2.4] - 2022-07-15

### 🚀 Features

- Add light theme

### 🐛 Bug Fixes

- Fix github action
- Fix github action
- Disable electron-builder publish

## [1.2.3] - 2022-06-15

### 🐛 Bug Fixes

- Update packages
- Change API to https

## [1.2.2] - 2022-03-20

### 🚀 Features

- Add export feature for gift data
- Add export option for gifts close #3

<!-- generated by git-cliff -->
