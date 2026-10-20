<p align="center">
  <img src="crates/cadence/assets/cadence.svg" width="96" alt="Cadence icon">
</p>

<h1 align="center">Cadence</h1>

<p align="center">
  <strong>使用 Rust 和 GPUI / GPUI Component 构建的原生 QQ 音乐桌面客户端</strong>
</p>

## 功能

1. 扫码登录，加载“我喜欢”和用户歌单，双击或点击行号播放；
2. 自由选择音质，缓存播过的曲目防止多次请求；
3. 支持“专属雷达”、“猜你喜欢”等智能歌单；
4. 支持搜索，查看单曲、歌单、专辑、歌手；
5. 支持歌词展示，内置 Ayu 亮色/暗色主题并可调整界面字体；
6. 支持自定义毛玻璃背景图片，可调模糊半径与面板不透明度，歌单、搜索和歌手页共用同一套玻璃背景；
7. 支持托盘，关闭窗口后仍在后台播放；
8. 记住上次窗口尺寸，启动时沿用；
9. 不依赖 WebView，运行流畅、占用较低。

## 项目结构

| 路径 | 内容 |
| --- | --- |
| `crates/cadence` | 桌面应用：GPUI 界面、播放、缓存、托盘、MPRIS |
| `crates/cadence-api` | QQ 音乐协议、登录和响应模型 |
| `crates/cadence-decrypt` | QMC 音频解密 |
| `vendor/gpui` | 带局部重绘和字体补丁的 GPUI 源码快照 |
| `packaging` | Linux 打包脚本、desktop 文件和图标 |

## 文档

- [代码结构与维护约束](docs/architecture.md)
- [GPUI 快照说明](vendor/README.md)
- [第三方许可](THIRD_PARTY_NOTICES.md)

## 构建

需要 Rust 1.98.1（由 `rust-toolchain.toml` 固定）以及 Linux 下的 GPUI 构建依赖。GPUI 源码已放在 `vendor/gpui`，构建时不需要再拉取 Zed 仓库。

```bash
sudo apt install build-essential clang cmake libasound2-dev libfontconfig1-dev \
  libglib2.0-dev libwayland-dev libx11-xcb-dev libxkbcommon-x11-dev \
  libzstd-dev libvulkan1 pkg-config
cargo build --release
./target/release/cadence
```

日常检查：

```bash
cargo check --locked -p cadence --all-targets
cargo test --locked -p cadence
cargo test --locked -p cadence-api
cargo clippy --locked -p cadence --all-targets
```

打包 deb（使用系统自带的 `dpkg-deb`，不需要 nfpm）：

```bash
packaging/build-deb.sh 0.1.0 amd64 dist
```

`packaging/build-packages.sh` 仍保留 nfpm 的 deb/rpm/Arch 全量流程。

## 安装

构建 deb 后安装：

```bash
sudo apt install ./dist/cadence-v0.1.0-linux-amd64.deb
```

也可以直接运行 `./target/release/cadence`。安装后可从应用菜单或终端运行 `cadence`。

## 发布

推送 `vX.Y.Z` 标签会触发 GitHub Actions 构建 Linux amd64/arm64 包，并创建 draft release。标签必须与 `Cargo.toml` 的 `workspace.package.version` 一致。

```bash
git tag v0.2.0
git push origin v0.2.0
```

CI 会生成 `.tar.gz`、`.deb`、`.rpm`、Arch 包、AppImage 和 `SHA256SUMS`，检查无误后在 GitHub Releases 页面手动发布。

## 平台支持

当前只提供 Linux 构建。程序使用跨平台的语言和 UI 框架，Windows / macOS 可以自行编译，但 GPUI 在这两个平台上缺少 Damage Tracking，歌词页在高强度重绘时性能不如 Linux。

## 参考与致谢

本项目基于 [amtoaer/lyrune](https://github.com/amtoaer/lyrune) 修改，实现过程中还参考了以下项目的逻辑：

+ [AstronW/netease-qq-music-api](https://github.com/AstronW/netease-qq-music-api)
+ [L-1124/QQMusicApi](https://github.com/L-1124/QQMusicApi)
+ [yakult-green-tea/qq-music-api](https://github.com/yakult-green-tea/qq-music-api)
+ [CharlesPikachu/musicdl](https://github.com/CharlesPikachu/musicdl)
+ [Yyyangshenghao/simple-music](https://github.com/Yyyangshenghao/simple-music)
+ [jixunmoe-go/qrc](https://github.com/jixunmoe-go/qrc)
+ [WXRIW/Lyricify-Lyrics-Helper](https://github.com/WXRIW/Lyricify-Lyrics-Helper)
+ [christosk92/WaveeMusic](https://github.com/christosk92/WaveeMusic)
+ [zed-industries/zed](https://github.com/zed-industries/zed)（GPUI 与 GPUI Component 的基础）
+ [longbridge/gpui-kit](https://github.com/longbridge/gpui-kit)（GPUI 组件库与快照导出脚本）
+ ...

感谢他们的贡献。

## 许可与免责声明

Cadence 是非官方客户端，与腾讯及 QQ 音乐无隶属或认可关系。请遵守当地法律、QQ 音乐服务条款及内容授权要求。

本项目是个人维护的非官方修改版本。原始代码来自 Lyrune；截至 2026-08-15，上游代码未声明开源许可，相关版权仍归其作者所有，再分发前请先确认上游授权。GPUI 以 Apache-2.0 快照形式放在 `vendor/gpui`，第三方组件的完整许可信息见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。
