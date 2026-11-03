import importlib.util
import unittest
from pathlib import Path

RUNTIME = Path(__file__).resolve().parents[1] / "runtime" / "orange.py"
spec = importlib.util.spec_from_file_location("orange_runtime", RUNTIME)
orange = importlib.util.module_from_spec(spec)
spec.loader.exec_module(orange)

class OrangeCodeTests(unittest.TestCase):
    def test_builtin_and_variable_translation(self):
        code = orange.translate('변수 이름 = "설이"\n출력(이름)\n숫자 = 정수("12")')
        self.assertIn('이름 = "설이"', code)
        self.assertIn("print", code)
        self.assertIn("int", code)

    def test_string_and_comment_are_not_translated(self):
        code = orange.translate('출력("만약 출력 참") # 출력은 한국어')
        self.assertIn('"만약 출력 참"', code)
        self.assertIn("# 출력은 한국어", code)

    def test_if_else_brace(self):
        code = orange.translate('만약 참 라면 {\n출력("맞음")\n} 아니면 {\n출력("아님")\n}')
        self.assertIn("if True:", code)
        self.assertIn("else:", code)

    def test_loop_translation(self):
        code = orange.translate("반복하기 3번 {\n출력(1)\n}")
        self.assertIn("for", code)
        self.assertIn("range", code)

    def test_brace_translation_preserves_source_line_numbers(self):
        source = '만약 참 라면 {\n출력("첫 줄")\n}\n출력("마지막 줄")'
        translated = orange.translate(source)
        self.assertEqual(len(source.splitlines()), len(translated.splitlines()))

    def test_python_style_indentation(self):
        code = orange.translate('만약 참:\n    출력("성공")\n아니면:\n    출력("실패")')
        self.assertIn("if True:", code)
        self.assertIn("    print", code)
        compile(code, "<test>", "exec")

    def test_functions_and_classes(self):
        code = orange.translate("함수 인사(이름) {\n반환 이름\n}")
        self.assertIn("def 인사(이름):", code)
        self.assertIn("return 이름", code)

if __name__ == "__main__":
    unittest.main()
