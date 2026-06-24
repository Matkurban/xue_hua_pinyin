# Changelog

本项目的所有重要变更均记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## 1.0.0

### 新增

- 基于 Rust [`pinyin`](https://crates.io/crates/pinyin) 0.11 实现核心拼音转换
- `toLazyPinyin`：汉字转拼音，每字取一个读音，返回 `List<String>`
- `toPinyin`：汉字转拼音，支持多音字，返回 `List<List<String>>`
- `PinyinStyle` 枚举：支持 `normal`、`tone`、`tone2`、`firstLetter`、`initials`、`finals`、`finalsTone`、`finalsTone2` 八种输出风格
- `PinyinArgs` 参数结构体：配置拼音风格与多音字开关
- `getFirstLetter`：获取单个字符串首字母
  - 中文 → 无声调拼音首字母大写
  - 英文 → 首字母大写
  - 数字 / 标点 / 其他字符 → 返回 `digitFallback`（默认 `#`）
  - 混合字符串仅根据第一个字符判定
- `getFirstLetters`：批量获取首字母
- `XueHuaPinyin.initialize()`：插件初始化入口
- 跨平台 FFI 支持：Android、iOS、macOS、Windows、Linux
- Rust 单元测试与 Flutter 集成测试
