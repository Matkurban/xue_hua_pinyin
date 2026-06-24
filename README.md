# xue_hua_pinyin

基于 Rust [pinyin](https://crates.io/crates/pinyin) 与 [flutter_rust_bridge](https://pub.dev/packages/flutter_rust_bridge) 的高性能 Flutter 拼音插件，适用于通讯录索引、头像占位、搜索分组等场景。

## 特性

- **核心拼音转换**：支持无声调、带声调、数字声调、首字母等多种输出风格，支持多音字
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
  xue_hua_pinyin: ^1.0.0
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
getFirstLetter(text: '张三');   // 'Z'

// 英文：取首字母并大写
getFirstLetter(text: 'Apple');  // 'A'

// 数字：默认返回 '#'
getFirstLetter(text: '123');    // '#'

// 自定义数字/标点占位符
getFirstLetter(text: '123', digitFallback: '*'); // '*'

// 批量处理
getFirstLetters(texts: ['张三', 'Bob', '007']); // ['Z', 'B', '#']
```

### 3. 拼音转换

```dart
// 无声调拼音，每字取一个读音
final lazy = toLazyPinyin(
  text: '中国人',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: false,
  ),
);
// ['zhong', 'guo', 'ren']

// 多音字：返回每个字的所有读音
final multi = toPinyin(
  text: '银行',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: true,
  ),
);
// [['yin', 'hang'], ['xing'], ...]（具体取决于多音字数据）
```

## API 参考

### 初始化

| 方法 | 说明 |
|------|------|
| `XueHuaPinyin.initialize()` | 初始化 Rust 运行时，应用启动时调用一次 |

### 首字母

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `getFirstLetter` | `text`（必填）、`digitFallback`（可选，默认 `'#'`） | `String` | 获取单个字符串的首字母 |
| `getFirstLetters` | `texts`（必填）、`digitFallback`（可选，默认 `'#'`） | `List<String>` | 批量获取首字母 |

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
getFirstLetter(text: '张3abc'); // 'Z' — 首字为中文
getFirstLetter(text: 'A张三');  // 'A' — 首字为英文
getFirstLetter(text: '1张三');  // '#' — 首字为数字
```

### 拼音转换

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `toLazyPinyin` | `text`、`args` | `List<String>` | 每字取一个读音 |
| `toPinyin` | `text`、`args` | `List<List<String>>` | 支持多音字，每字可返回多个读音 |

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
  final letter = getFirstLetter(text: name);
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
  return getFirstLetter(text: name);
}

avatarText('张三');  // 'Z'
avatarText('Tom');   // 'T'
avatarText('007');   // '#'
```

## 开发与构建

本项目使用 [flutter_rust_bridge](https://codelabs.developers.google.com/codelabs/flutter-codelab-frb) 进行 Dart ↔ Rust 绑定。

### 项目结构

```
xue_hua_pinyin/
├── lib/
│   ├── xue_hua_pinyin.dart          # 公共导出
│   └── src/
│       ├── xue_hua_pinyin.dart      # XueHuaPinyin.initialize()
│       └── rust/                    # FRB 生成的 Dart 绑定
├── rust/
│   └── src/api/
│       ├── pinyin_api.rs            # 拼音 API 实现
│       └── simple.rs                # FRB 初始化
├── flutter_rust_bridge.yaml         # FRB 代码生成配置
└── cargokit/                        # 跨平台 Rust 构建工具
```

### 修改 Rust API 后重新生成绑定

```bash
flutter_rust_bridge_codegen generate
```

### 运行 Rust 单元测试

```bash
cd rust && cargo test
```

### 运行集成测试

```bash
cd example && flutter test integration_test/simple_test.dart
```

## 依赖说明

| 依赖 | 用途 |
|------|------|
| [pinyin](https://crates.io/crates/pinyin) 0.11 | Rust 拼音转换引擎 |
| [flutter_rust_bridge](https://pub.dev/packages/flutter_rust_bridge) 2.12.0 | Dart ↔ Rust FFI 绑定 |

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 发布。
