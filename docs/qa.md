---
title: 常见问题
description: 安装、直播间连接、软件更新与插件使用中的常见问题。
---

## 从哪里下载？

前往 [GitHub Releases](https://github.com/powerlive-io/JLiverTool/releases)，根据操作系统和处理器选择安装包。初次连接直播间的步骤见[开始使用](/setup)。

## macOS 提示“文件已损坏”怎么办？

应用尚未经过 Apple 签名。如果你从项目发布页下载了应用，并已将其放入“应用程序”目录，可以在终端执行：

```bash
xattr -cr /Applications/JLiverTool.app
```

然后重新打开 JLiverTool。

## 为什么不能修改直播间标题或开关播？

这些操作需要登录账号与直播间匹配。请在设置中检查当前登录账号和填写的直播间号。

## 如何检查更新？

打开“设置 → 关于 → 更新”，点击“检查更新”。发现新版本后，可以选择“下载并更新”或“查看更新内容”。开发构建和 Arch 软件包请手动更新，详情见[软件更新](/setup#软件更新)。

## 为什么插件没有出现在列表里？

确认插件目录位于应用数据目录的 `plugins/` 下，包含有效的 `meta.json`，且 `index` 指向的入口文件存在。然后在插件管理页面点击“刷新”。

可以从“设置 → 插件管理 → 插件说明”打开插件目录。完整说明见[安装插件](/plugin-document#安装插件)。

## 为什么插件收不到直播事件？

从 JLiverTool 的插件管理页面点击“打开”，由应用内置服务器提供插件页面并注入 `jliverAPI`。直接通过 `file://` 打开的网页不会被注入 API。

如果修改了插件 HTTP 或 WebSocket 端口，需要重启应用。也请确认主窗口已经连接到目标直播间。

## 如何反馈问题？

在 [GitHub Issues](https://github.com/powerlive-io/JLiverTool/issues)中提供操作系统、应用版本、复现步骤及错误现象。附上截图前，请遮盖账号隐私信息。
