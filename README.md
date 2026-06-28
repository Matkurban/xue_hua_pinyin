# xue_hua_pinyin

> **Language:** [中文文档](README.zh-CN.md)

A high-performance Flutter pinyin plugin powered by Rust. Converts Chinese to pinyin (heteronym, multiple tone styles) and extracts index letters for contact lists, avatar placeholders, search grouping, and similar scenarios.

## Features

- **Core pinyin conversion**: toneless, toned, numeric tone, first letter, initials, finals, and more
- **Heteronym support**: return all readings per character via `toPinyin`
- **Index letters**: single and batch APIs for mixed Chinese/English strings
- **High performance**: core logic in Rust with synchronous calls
- **Cross-platform**: Android, iOS, macOS, Windows, Linux

## Platform Support

| Platform | Supported |
|----------|-----------|
| Android  | Yes       |
| iOS      | Yes       |
| macOS    | Yes       |
| Windows  | Yes       |
| Linux    | Yes       |
| Web      | No        |

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  xue_hua_pinyin: ^lasted
```

## Quick Start

### 1. Initialize

Call `XueHuaPinyin.initialize()` once before using any other API:

```dart
import 'package:xue_hua_pinyin/xue_hua_pinyin.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await XueHuaPinyin.initialize();
  runApp(const MyApp());
}
```

> **Note:** `XueHuaPinyin.initialize()` is safe to call multiple times, but only needs to run once at app startup.

### 2. Index Letters

Useful for contact grouping, avatar placeholder text, and similar UI:

```dart
// Chinese: uppercase first letter of toneless pinyin
XueHuaPinyin.getFirstLetter(text: '张三');   // 'Z'

// English: uppercase first letter
XueHuaPinyin.getFirstLetter(text: 'Apple');  // 'A'

// Digits: default fallback '#'
XueHuaPinyin.getFirstLetter(text: '123');    // '#'

// Custom fallback for digits / punctuation
XueHuaPinyin.getFirstLetter(text: '123', digitFallback: '*'); // '*'

// Batch
XueHuaPinyin.getFirstLetters(texts: ['张三', 'Bob', '007']); // ['Z', 'B', '#']
```

### 3. Pinyin Conversion

```dart
// Toneless pinyin, one reading per character
final lazy = XueHuaPinyin.toLazyPinyin(
  text: '中国人',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: false,
  ),
);
// ['zhong', 'guo', 'ren']

// Heteronym: all readings per character
final multi = XueHuaPinyin.toPinyin(
  text: '银行',
  args: const PinyinArgs(
    style: PinyinStyle.normal,
    heteronym: true,
  ),
);
// [['yin', 'hang'], ['xing'], ...] (depends on heteronym data)
```

## API Reference

All methods are static members of [`XueHuaPinyin`](lib/src/xue_hua_pinyin.dart). Types [`PinyinArgs`](lib/src/rust/api/pinyin_api.dart) and [`PinyinStyle`](lib/src/rust/api/pinyin_api.dart) are exported at the package level for constructing arguments.

### Initialization

| Method | Description |
|--------|-------------|
| `XueHuaPinyin.initialize()` | Initialize the Rust runtime; call once at startup |
| `XueHuaPinyin.defaultPinyinArgs()` | Returns default `PinyinArgs` from Rust |

### Index Letters

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `XueHuaPinyin.getFirstLetter` | `text` (required), `digitFallback` (optional, default `'#'`) | `String` | Index letter for one string |
| `XueHuaPinyin.getFirstLetters` | `texts` (required), `digitFallback` (optional, default `'#'`) | `List<String>` | Index letters for many strings |

#### Index Letter Rules

The **first character** of `text.trim()` determines the result:

| First character | Behavior | Example |
|-----------------|----------|---------|
| Chinese hanzi | Toneless pinyin first letter, uppercased | `'张三'` → `'Z'` |
| ASCII letter | Uppercased | `'Apple'` → `'A'` |
| Digit | Returns `digitFallback` (default `'#'`) | `'123'` → `'#'` |
| Punctuation | Returns `digitFallback` (default `'#'`) | `'!hello'` → `'#'` |
| Other | Returns `digitFallback` (default `'#'`) | emoji → `'#'` |
| Empty string | Empty string | `''` → `''` |

#### Mixed Strings

Only the **first character** matters:

```dart
XueHuaPinyin.getFirstLetter(text: '张3abc'); // 'Z' — first char is Chinese
XueHuaPinyin.getFirstLetter(text: 'A张三');  // 'A' — first char is English
XueHuaPinyin.getFirstLetter(text: '1张三');  // '#' — first char is digit
```

### Pinyin Conversion

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `XueHuaPinyin.toLazyPinyin` | `text`, `args` | `List<String>` | One reading per character |
| `XueHuaPinyin.toPinyin` | `text`, `args` | `List<List<String>>` | Optional heteronym; multiple readings per character |

#### PinyinArgs

| Field | Type | Description |
|-------|------|-------------|
| `style` | `PinyinStyle` | Output style |
| `heteronym` | `bool` | Enable heteronym (only affects `toPinyin`) |

#### PinyinStyle

| Value | Description | Example |
|-------|-------------|---------|
| `normal` | Toneless | `zhong` |
| `tone` | With tone marks | `zhōng` |
| `tone2` | Numeric tone (mid-syllable) | `zho1ng` |
| `firstLetter` | First letter | `z` |
| `initials` | Initial consonant | `zh` |
| `finals` | Final (toneless) | `ong` |
| `finalsTone` | Final with tone | `ōng` |
| `finalsTone2` | Final with numeric tone | `o1ng` |

## Usage Examples

### Contact List Grouping

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

### Avatar Placeholder Text

```dart
String avatarText(String? name) {
  if (name == null || name.trim().isEmpty) return '';
  return XueHuaPinyin.getFirstLetter(text: name);
}

avatarText('张三');  // 'Z'
avatarText('Tom');   // 'T'
avatarText('007');   // '#'
```


## Dependencies

| Dependency | Purpose |
|------------|---------|
| [pinyin](https://crates.io/crates/pinyin) 0.11 | Rust pinyin engine |
| [flutter_rust_bridge](https://pub.dev/packages/flutter_rust_bridge) 2.12.0 | Dart ↔ Rust FFI |

## License

Licensed under the [Apache License 2.0](LICENSE).
