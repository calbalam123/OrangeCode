import importlib.util
import unittest
from pathlib import Path

RUNTIME = Path(__file__).resolve().parents[1] / "runtime" / "orange.py"
spec = importlib.util.spec_from_file_location("orange_runtime", RUNTIME)
orange = importlib.util.module_from_spec(spec)
spec.loader.exec_module(orange)

class OrangeCodeTests(unittest.TestCase):
    def test_variable_and_output_translation(self):
        code = orange.translate('변수 이름 = "설이"\n출력하기(이름)')
        self.assertIn('이름 = "설이"', code)
        self.assertIn("출력하기(이름)", code)

    def test_if_else_inline_brace(self):
        code = orange.translate('만약 참 라면 {\n출력하기("맞음")\n} 아니면 {\n출력하기("아님")\n}')
        self.assertIn("if True:", code)
        self.assertIn("else:", code)

    def test_loop_translation(self):
        code = orange.translate("반복하기 3번 {\n출력하기(1)\n}")
        self.assertIn("range(int(3))", code)

    def test_function_translation(self):
        code = orange.translate("함수 인사(이름) {\n반환 이름\n}")
        self.assertIn("def 인사(이름):", code)
        self.assertIn("return 이름", code)

if __name__ == "__main__":
    unittest.main()
