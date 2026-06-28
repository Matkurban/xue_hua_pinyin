# xue_hua_pinyin

> **Language / 语言：** [English README](README.md)

基于 Rust 的高性能 Flutter 拼音插件。支持汉字转拼音（多音字、多种声调风格）以及通讯录索引字母提取，适用于通讯录分组、头像占位、搜索分组等场景。

## 特性

- **核心拼音转换**：无声调、带声调、数字声调、首字母、声母、韵母等多种输出风格
- **多音字支持**：通过 `toPinyin` 返回每个汉字的全部读音
- **首字母便捷方法**：提供单个与批量 API，自动处理中英文混合字符串
- **高性能**：核心逻辑由 Rust 实现，同步调用、开销低
- **跨平台**：支持 Android、iOS、macOS、Windows、Linux

## 平台支持

| 平台    | 支持 |
|---------|------|
| Android | ✅   |
| iOS     | ✅   |
| macOS   | ✅   |
| Windows | ✅   |
| Linux   | ✅   |
| Web     | ❌   |

## 安装

在 `pubspec.yaml` 中添加依赖：

```yaml
dependencies:
  xue_hua_pinyin: ^lasted
```

## 快速开始

### 1. 初始化

在使用任何 API 之前，必须先调用一次初始化方法：

```dart
import 'package:xue_hua_pinyin/xue_hua_pinyin.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await XueHuaPinyin.initialize();
  runApp(const MyApp());
}
```

> **注意**：`XueHuaPinyin.initialize()` 只需在应用启动时调用一次，重复调用是安全的。

### 2. 获取首字母

适用于通讯录分组、头像占位文字等场景：

```dart
// 中文：取无声调拼音首字母并大写
XueHuaPinyin.getFirstLetter(text: '张三');   // 'Z'

// 英文：取首字母并大写
XueHuaPinyin.getFirstLetter(text: 'Apple');  // 'A'

// 数字：默认返回 '#'
XueHuaPinyin.getFirstLetter(text: '123');    // '#'

// 自定义数字/标点占位符
XueHuaPinyin.getFirstLetter(text: '123', digitFallback: '*'); // '*'

// 批量处理
XueHuaPinyin.getFirstLetters(texts: ['张三', 'Bob', '007']); // ['Z', 'B', '#']
```

### 3. 拼音转换

```dart
// 无声调拼音，每字取一个读音
final lazy = XueHuaPinyin.toLazyPinyin(
  text: '中国人',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: false,
  ),
);
// ['zhong', 'guo', 'ren']

// 多音字：返回每个字的所有读音
final multi = XueHuaPinyin.toPinyin(
  text: '银行',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: true,
  ),
);
// [['yin', 'hang'], ['xing'], ...]（具体取决于多音字数据）
```

## API 参考

所有方法均为 [`XueHuaPinyin`](lib/src/xue_hua_pinyin.dart) 的静态成员。参数类型 [`PinyinArgs`](lib/src/rust/api/pinyin_api.dart) 与 [`PinyinStyle`](lib/src/rust/api/pinyin_api.dart) 在包顶层导出，便于构造参数。

### 初始化

| 方法 | 说明 |
|------|------|
| `XueHuaPinyin.initialize()` | 初始化 Rust 运行时，应用启动时调用一次 |
| `XueHuaPinyin.defaultPinyinArgs()` | 从 Rust 获取默认 `PinyinArgs` |

### 首字母

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `XueHuaPinyin.getFirstLetter` | `text`（必填）、`digitFallback`（可选，默认 `'#'`） | `String` | 获取单个字符串的首字母 |
| `XueHuaPinyin.getFirstLetters` | `texts`（必填）、`digitFallback`（可选，默认 `'#'`） | `List<String>` | 批量获取首字母 |

#### 首字母判定规则

对 `text.trim()` 后的**第一个字符**进行判定：

| 首字符类型 | 处理方式 | 示例 |
|------------|----------|------|
| 中文汉字 | 转为无声调拼音，取首字母并大写 | `'张三'` → `'Z'` |
| 英文字母 | 直接取首字母并大写 | `'Apple'` → `'A'` |
| 数字 | 返回 `digitFallback`（默认 `'#'`） | `'123'` → `'#'` |
| 标点符号 | 返回 `digitFallback`（默认 `'#'`） | `'!hello'` → `'#'` |
| 其他字符 | 返回 `digitFallback`（默认 `'#'`） | emoji 等 → `'#'` |
| 空字符串 | 返回空字符串 | `''` → `''` |

#### 混合字符串

只根据**第一个字符**决定结果，后续字符不影响输出：

```dart
XueHuaPinyin.getFirstLetter(text: '张3abc'); // 'Z' — 首字为中文
XueHuaPinyin.getFirstLetter(text: 'A张三');  // 'A' — 首字为英文
XueHuaPinyin.getFirstLetter(text: '1张三');  // '#' — 首字为数字
```

### 拼音转换

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `XueHuaPinyin.toLazyPinyin` | `text`、`args` | `List<String>` | 每字取一个读音 |
| `XueHuaPinyin.toPinyin` | `text`、`args` | `List<List<String>>` | 支持多音字，每字可返回多个读音 |

#### PinyinArgs

| 字段 | 类型 | 说明 |
|------|------|------|
| `style` | `PinyinStyle` | 拼音输出风格 |
| `heteronym` | `bool` | 是否启用多音字（仅 `toPinyin` 有效） |

#### PinyinStyle

| 值 | 说明 | 示例 |
|----|------|------|
| `normal` | 无声调 | `zhong` |
| `tone` | 带声调 | `zhōng` |
| `tone2` | 数字声调（声调在字母中间） | `zho1ng` |
| `firstLetter` | 首字母 | `z` |
| `initials` | 声母 | `zh` |
| `finals` | 韵母 | `ong` |
| `finalsTone` | 带声调韵母 | `ōng` |
| `finalsTone2` | 数字声调韵母 | `o1ng` |

## 使用场景示例

### 通讯录索引分组

```dart
String sectionKey(String name) {
  final letter = XueHuaPinyin.getFirstLetter(text: name);
  if (letter == '#') return '#';
  return letter; // 'A' ~ 'Z'
}

final names = ['Alice', '张三', 'Bob', '123号', '!特殊'];
final groups = <String, List<String>>{};

for (final name in names) {
  final key = sectionKey(name);
  groups.putIfAbsent(key, () => []).add(name);
}
// {'A': ['Alice'], 'Z': ['张三'], 'B': ['Bob'], '#': ['123号', '!特殊']}
```

### 头像占位文字

```dart
String avatarText(String? name) {
  if (name == null || name.trim().isEmpty) return '';
  return XueHuaPinyin.getFirstLetter(text: name);
}

avatarText('张三');  // 'Z'
avatarText('Tom');   // 'T'
avatarText('007');   // '#'
```

## 依赖说明

| 依赖 | 用途 |
|------|------|
| [pinyin](https://crates.io/crates/pinyin) 0.11 | Rust 拼音转换引擎 |
| [flutter_rust_bridge](https://pub.dev/packages/flutter_rust_bridge) 2.12.0 | Dart ↔ Rust FFI 绑定 |

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 发布。
