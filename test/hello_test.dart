import 'package:test/test.dart';
import '../lib/ffi.dart';

void main() {
  test('Should greet', () async {
    final greeting = await api.helloWorld();
    const expected = "Hello, world!";
    expect(greeting, expected);
  });
}
