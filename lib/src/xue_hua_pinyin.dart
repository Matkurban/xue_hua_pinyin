import 'rust/api/pinyin_api.dart' show PinyinArgs;
import 'rust/api/pinyin_api.dart' as pinyin_api hide PinyinArgs, PinyinStyle;
import 'rust/frb_generated.dart';

export 'rust/api/pinyin_api.dart' show PinyinArgs, PinyinStyle;

/// Entry point for the xue_hua_pinyin plugin.
class XueHuaPinyin {
  XueHuaPinyin._();

  /// Initializes the Rust runtime. Must be called once before any other API.
  static Future<void> initialize() => RustLib.init();

  /// Returns the default [PinyinArgs] from Rust.
  static Future<PinyinArgs> defaultPinyinArgs() => PinyinArgs.default_();

  /// Returns the index letter for a single string.
  static String getFirstLetter({required String text, String? digitFallback}) =>
      pinyin_api.getFirstLetter(text: text, digitFallback: digitFallback);

  /// Returns index letters for multiple strings.
  static List<String> getFirstLetters({
    required List<String> texts,
    String? digitFallback,
  }) => pinyin_api.getFirstLetters(texts: texts, digitFallback: digitFallback);

  /// Converts Chinese text to pinyin, one reading per character.
  static List<String> toLazyPinyin({
    required String text,
    required PinyinArgs args,
  }) => pinyin_api.toLazyPinyin(text: text, args: args);

  /// Converts Chinese text to pinyin with optional heteronym support.
  static List<List<String>> toPinyin({
    required String text,
    required PinyinArgs args,
  }) => pinyin_api.toPinyin(text: text, args: args);
}
