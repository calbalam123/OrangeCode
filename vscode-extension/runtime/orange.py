#!/usr/bin/env python3
"""Orange Code 1.0.0 - Korean-style educational language runtime."""
from __future__ import annotations
import argparse
import re
import sys
from pathlib import Path

VERSION = "1.0.0"

def convert_expression(expr: str) -> str:
    expr = expr.strip()
    expr = re.sub(r"\b참\b", "True", expr)
    expr = re.sub(r"\b거짓\b", "False", expr)
    expr = re.sub(r"\b그리고\b", "and", expr)
    expr = re.sub(r"\b또는\b", "or", expr)
    expr = re.sub(r"\b아니다\b", "not", expr)
    return expr

def translate(source: str) -> str:
    output = []
    indent = 0
    for line_no, original in enumerate(source.splitlines(), 1):
        line = original.strip()
        if not line or line.startswith("#"):
            continue
        if line == "}":
            indent -= 1
            if indent < 0:
                raise SyntaxError(f"{line_no}행: 닫는 중괄호가 너무 많습니다.")
            continue
        if line == "} 아니면 {":
            indent -= 1
            if indent < 0:
                raise SyntaxError(f"{line_no}행: 닫는 중괄호가 너무 많습니다.")
            output.append("    " * indent + "else:")
            indent += 1
            continue

        statement = None
        match = re.fullmatch(r"만약\s+(.+?)\s+라면\s*\{", line)
        if match:
            statement = f"if {convert_expression(match.group(1))}:"
        else:
            match = re.fullmatch(r"아니면\s*\{", line)
            if match:
                statement = "else:"
            else:
                match = re.fullmatch(r"반복하기\s+(.+?)번\s*\{", line)
                if match:
                    statement = f"for _orange_index in range(int({convert_expression(match.group(1))})):"
                else:
                    match = re.fullmatch(r"각각\s+(\w+)\s+안에\s+(.+?)\s+반복하기\s*\{", line)
                    if match:
                        statement = f"for {match.group(1)} in {convert_expression(match.group(2))}:"
                    else:
                        match = re.fullmatch(r"동안\s+(.+?)\s+반복하기\s*\{", line)
                        if match:
                            statement = f"while {convert_expression(match.group(1))}:"
                        else:
                            match = re.fullmatch(r"함수\s+(\w+)\s*\((.*?)\)\s*\{", line)
                            if match:
                                statement = f"def {match.group(1)}({convert_expression(match.group(2))}):"

        if statement is not None:
            output.append("    " * indent + statement)
            indent += 1
            continue

        if line.endswith("{") or "{" in line or "}" in line:
            raise SyntaxError(f"{line_no}행: 블록 문법을 확인하세요. 여는 중괄호는 조건/반복/함수 문장 끝에 둡니다.")

        match = re.match(r"변수\s+([\w가-힣]+)\s*=(?!=)\s*(.*)$", line)
        if match:
            line = f"{match.group(1)} = {convert_expression(match.group(2))}"
        elif line.startswith("반환 "):
            line = "return " + convert_expression(line[3:])
        elif line == "반환":
            line = "return"
        else:
            line = convert_expression(line)

        output.append("    " * indent + line)

    if indent != 0:
        raise SyntaxError("닫히지 않은 중괄호가 있습니다. 모든 { 블록에 }를 추가하세요.")
    return "\n".join(output) + "\n"

def make_environment():
    def add_to_list(items, value):
        items.append(value)
        return items
    def remove_from_list(items, value):
        if value in items:
            items.remove(value)
        return items
    return {
        "__name__": "__orange__",
        "출력하기": print,
        "입력하기": input,
        "길이": len,
        "문자열로": str,
        "숫자로": float,
        "정수로": int,
        "절댓값": abs,
        "반올림": round,
        "최대": max,
        "최소": min,
        "추가하기": add_to_list,
        "삭제하기": remove_from_list,
        "포함하는가": lambda items, value: value in items,
        "범위": range,
        "참": True,
        "거짓": False,
    }

def run_file(path: Path) -> int:
    try:
        source = path.read_text(encoding="utf-8-sig")
        translated = translate(source)
        code = compile(translated, str(path), "exec")
        env = make_environment()
        exec(code, env, env)
        return 0
    except FileNotFoundError:
        print(f"[오렌지코드 오류] 파일을 찾을 수 없습니다: {path}", file=sys.stderr)
    except SyntaxError as exc:
        print(f"[오렌지코드 문법 오류] {exc}", file=sys.stderr)
    except Exception as exc:
        print(f"[오렌지코드 실행 오류] {type(exc).__name__}: {exc}", file=sys.stderr)
    return 1

def main():
    parser = argparse.ArgumentParser(description="오렌지코드 한국어 프로그래밍 언어 실행기")
    parser.add_argument("--version", action="version", version=f"Orange Code {VERSION}")
    parser.add_argument("file", nargs="?", help="실행할 .orange 파일")
    args = parser.parse_args()
    if not args.file:
        parser.print_help()
        return 2
    return run_file(Path(args.file))

if __name__ == "__main__":
    raise SystemExit(main())
