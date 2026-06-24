import 'rust/frb_generated.dart';

/// 雪花拼音插件入口。
class XueHuaPinyin {
  XueHuaPinyin._();

  /// 初始化 Rust 运行时，使用前必须调用一次。
  static Future<void> initialize() => RustLib.init();
}
