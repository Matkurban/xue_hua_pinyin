import 'package:flutter/material.dart';
import 'package:xue_hua_pinyin/xue_hua_pinyin.dart';

Future<void> main() async {
  await XueHuaPinyin.initialize();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final firstLetter = getFirstLetter(text: '张三');
    final pinyin = toLazyPinyin(
      text: '中国人',
      args: const PinyinArgs(style: PinyinStyle.normal, heteronym: false),
    );

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('xue_hua_pinyin')),
        body: Center(
          child: Text(
            '首字母: $firstLetter\n'
            '拼音: ${pinyin.join(' ')}',
          ),
        ),
      ),
    );
  }
}
