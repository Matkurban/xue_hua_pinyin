import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:xue_hua_pinyin/xue_hua_pinyin.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await XueHuaPinyin.initialize());

  test('getFirstLetter converts Chinese to pinyin initial', () {
    expect(XueHuaPinyin.getFirstLetter(text: '张三'), 'Z');
  });

  test('getFirstLetter returns uppercase English letter', () {
    expect(XueHuaPinyin.getFirstLetter(text: 'Apple'), 'A');
  });

  test('getFirstLetter returns default # for digits', () {
    expect(XueHuaPinyin.getFirstLetter(text: '123'), '#');
  });

  test('getFirstLetter supports custom digit fallback', () {
    expect(XueHuaPinyin.getFirstLetter(text: '123', digitFallback: '*'), '*');
  });

  test('getFirstLetter uses first character for mixed strings', () {
    expect(XueHuaPinyin.getFirstLetter(text: '张3abc'), 'Z');
    expect(XueHuaPinyin.getFirstLetter(text: 'A张三'), 'A');
    expect(XueHuaPinyin.getFirstLetter(text: '1张三'), '#');
  });

  test('getFirstLetter returns fallback for punctuation', () {
    expect(XueHuaPinyin.getFirstLetter(text: '!hello'), '#');
    expect(XueHuaPinyin.getFirstLetter(text: '@test', digitFallback: '*'), '*');
  });

  test('getFirstLetters batch converts correctly', () {
    expect(XueHuaPinyin.getFirstLetters(texts: ['张三', 'Bob', '007']), [
      'Z',
      'B',
      '#',
    ]);
  });

  test('toLazyPinyin converts Chinese to toneless pinyin', () {
    expect(
      XueHuaPinyin.toLazyPinyin(
        text: '中国人',
        args: const PinyinArgs(style: PinyinStyle.normal, heteronym: false),
      ),
      ['zhong', 'guo', 'ren'],
    );
  });
}
