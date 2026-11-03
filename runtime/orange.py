#!/usr/bin/env python3
"""Orange Code 2.4.1 - Korean aliases for Python."""
from __future__ import annotations
import argparse, builtins, io, re, sys, tokenize
from pathlib import Path

# Keep Korean text readable on Windows consoles and VS Code terminals.
for _stream in (sys.stdin, sys.stdout, sys.stderr):
    if hasattr(_stream, "reconfigure"):
        try:
            _stream.reconfigure(encoding="utf-8")
        except (OSError, ValueError):
            pass
VERSION = "2.4.1"
NAME_MAP = {
"만약":"if","아니면":"else","아니면만약":"elif","반복":"for","동안":"while","함수":"def","클래스":"class","반환":"return",
"가져오기":"import","에서":"from","별칭":"as","시도":"try","예외":"except","마지막으로":"finally","발생":"raise","확인":"assert",
"아무것도안함":"pass","중단":"break","계속":"continue","그리고":"and","또는":"or","아니다":"not","안에":"in","같은가":"is",
"람다":"lambda","함께":"with","비동기":"async","기다리기":"await","생성":"yield","전역":"global","비지역":"nonlocal","삭제":"del",
"참":"True","거짓":"False","없음":"None","일치":"match","경우":"case",
"출력":"print","출력하기":"print","입력":"input","입력하기":"input","길이":"len","자료형":"type","식별값":"id","해시값":"hash",
"도움말":"help","목록":"list","튜플":"tuple","집합":"set","불변집합":"frozenset","사전":"dict","문자열":"str","바이트":"bytes",
"바이트배열":"bytearray","메모리뷰":"memoryview","정수":"int","실수":"float","복소수":"complex","논리값":"bool","범위":"range",
"열거":"enumerate","압축":"zip","지도":"map","필터":"filter","정렬":"sorted","역순":"reversed","합계":"sum","최대":"max","최소":"min",
"절댓값":"abs","반올림":"round","몫나머지":"divmod","거듭제곱":"pow","모든참":"all","하나라도참":"any","호출가능":"callable",
"문자코드":"ord","코드문자":"chr","진법문자열":"bin","팔진수문자열":"oct","십육진수문자열":"hex","형식화":"format",
"속성가져오기":"getattr","속성설정":"setattr","속성삭제":"delattr","속성있는가":"hasattr","인스턴스인가":"isinstance",
"하위클래스인가":"issubclass","객체생성":"object","상위호출":"super","정적메서드":"staticmethod","클래스메서드":"classmethod",
"속성":"property","열기":"open","메모":"repr","변수목록":"vars","이름목록":"dir","지역변수":"locals","전역변수":"globals",
"실행":"exec","평가":"eval","컴파일":"compile","중단점":"breakpoint","반복자":"iter","다음값":"next",
"문자열로":"str","정수로":"int","숫자로":"float","최솟값":"min","최댓값":"max","절대값":"abs","모두참인가":"all",
"하나라도참인가":"any","파일열기":"open","문자열표현":"repr","진리값":"bool","유니코드문자":"chr","문자번호":"ord",
"목록으로":"list","튜플로":"tuple","집합으로":"set","사전으로":"dict","범위로":"range","열거하기":"enumerate",
"묶기":"zip","정렬된":"sorted","뒤에서부터":"reversed","형식":"format","속성조회":"getattr","속성변경":"setattr",
"속성제거":"delattr","속성존재":"hasattr","이름공간":"vars","코드실행":"exec","식평가":"eval","코드컴파일":"compile",
"도움말보기":"help","해시":"hash","바이트로":"bytes","불변집합으로":"frozenset","부동소수점":"float","복소수로":"complex"
}
METHOD_MAP = {
"대문자로":"upper","소문자로":"lower","대문자화":"capitalize","제목형으로":"title","교체":"replace","나누기":"split",
"합치기":"join","제거":"strip","왼쪽제거":"lstrip","오른쪽제거":"rstrip","찾기":"find","개수":"count",
"시작하는가":"startswith","끝나는가":"endswith","추가하기":"append","확장하기":"extend","삽입하기":"insert",
"빼내기":"pop","지우기":"remove","비우기":"clear","뒤집기":"reverse","정렬하기":"sort","복사하기":"copy",
"키":"keys","값들":"values","항목":"items","가져오기값":"get","갱신하기":"update","인덱스":"index",
"대소문자무시비교":"casefold","왼쪽채우기":"ljust","오른쪽채우기":"rjust","가운데정렬":"center",
"접두사제거":"removeprefix","접미사제거":"removesuffix","분할하기":"partition","뒤에서분할":"rpartition",
"줄나누기":"splitlines","문자검사":"isalpha","숫자검사":"isdigit","영숫자검사":"isalnum","공백검사":"isspace",
"소문자인가":"islower","대문자인가":"isupper","제목형인가":"istitle","키목록":"keys","값목록":"values",
"항목목록":"items","기본값가져오기":"setdefault","항목제거":"popitem","원소추가":"add","원소제거":"discard",
"부분집합인가":"issubset","상위집합인가":"issuperset","교집합":"intersection","합집합":"union","차집합":"difference",
"대칭차집합":"symmetric_difference","개수세기":"count","복사":"copy","초기화":"clear"
}
def _replace_names(source):
    out=[]
    try:
        tokens=list(tokenize.generate_tokens(io.StringIO(source).readline))
        for i,t in enumerate(tokens):
            replacement=None
            if t.type==tokenize.NAME:
                if t.string in NAME_MAP: replacement=NAME_MAP[t.string]
                elif t.string in METHOD_MAP and i>0 and tokens[i-1].string==".": replacement=METHOD_MAP[t.string]
            if replacement is not None: t=tokenize.TokenInfo(t.type,replacement,t.start,t.end,t.line)
            out.append(t)
        return tokenize.untokenize(out)
    except tokenize.TokenError as exc: raise SyntaxError(f"코드 토큰 오류: {exc}") from exc
def translate(source):
    lines=[]; indent=0
    for no,original in enumerate(source.splitlines(),1):
        line=original.strip()
        if not line or line.startswith("#"): lines.append(original); continue
        if line=="}":
            indent-=1
            if indent<0: raise SyntaxError(f"{no}행: 닫는 중괄호가 너무 많습니다.")
            # Preserve source line numbers for VS Code debugging.
            lines.append("")
            continue
        if line.startswith("} 아니면"):
            indent-=1
            if indent<0: raise SyntaxError(f"{no}행: 닫는 중괄호가 너무 많습니다.")
            if line=="} 아니면 {": lines.append("    "*indent+"아니면:")
            else:
                cond=line[len("} 아니면만약 "):].strip().removesuffix("{").strip()
                lines.append("    "*indent+f"아니면만약 {cond}:")
            indent+=1; continue
        statement=None
        patterns=[
          (r"만약\s+(.+?)\s+라면\s*\{",lambda m:f"만약 {m[1]}:"),
          (r"아니면만약\s+(.+?)\s*(?:라면)?\s*\{",lambda m:f"아니면만약 {m[1]}:"),
          (r"아니면\s*\{",lambda m:"아니면:"),
          (r"반복하기\s+(.+?)번\s*\{",lambda m:f"반복 인덱스 안에 범위(정수({m[1]})):"),
          (r"각각\s+(\w+)\s+안에\s+(.+?)\s+반복하기\s*\{",lambda m:f"반복 {m[1]} 안에 {m[2]}:"),
          (r"동안\s+(.+?)\s+반복하기\s*\{",lambda m:f"동안 {m[1]}:"),
          (r"함수\s+([\w가-힣]+)\s*\((.*?)\)\s*\{",lambda m:f"함수 {m[1]}({m[2]}):"),
          (r"클래스\s+([\w가-힣]+)(?:\((.*?)\))?\s*\{",lambda m:f"클래스 {m[1]}({m[2] or ''}):")
        ]
        for pattern,build in patterns:
            m=re.fullmatch(pattern,line)
            if m: statement=build(m); break
        if statement is not None: lines.append("    "*indent+statement); indent+=1; continue
        if "{" in line or "}" in line: raise SyntaxError(f"{no}행: 블록 문법을 확인하세요.")
        m=re.match(r"변수\s+([\w가-힣]+)\s*=(?!=)\s*(.*)$",line)
        if m: line=f"{m[1]} = {m[2]}"
        source_indent=len(original)-len(original.lstrip(" \t"))
        lines.append("    "*indent+original[:source_indent]+line)
    if indent: raise SyntaxError("닫히지 않은 중괄호가 있습니다.")
    return _replace_names("\n".join(lines)+"\n")
def make_environment():
    env={"__name__":"__orange__"}
    for korean,english in NAME_MAP.items():
        if hasattr(builtins,english): env[korean]=getattr(builtins,english)
    env.update({"참":True,"거짓":False,"없음":None})
    env["포함하는가"]=lambda items,value:value in items
    return env
def run_file(path):
    try:
        src=path.read_text(encoding="utf-8-sig"); code=compile(translate(src),str(path),"exec")
        env=make_environment(); exec(code,env,env); return 0
    except FileNotFoundError: print(f"파일을 찾을 수 없습니다: {path}",file=sys.stderr)
    except SyntaxError as e: print(f"오렌지코드 문법 오류: {e}",file=sys.stderr)
    except Exception as e: print(f"오렌지코드 실행 오류: {type(e).__name__}: {e}",file=sys.stderr)
    return 1
def main():
    p=argparse.ArgumentParser(description="오렌지코드 한국어 프로그래밍 언어 실행기")
    p.add_argument("--version",action="version",version=f"Orange Code {VERSION}")
    p.add_argument("--translate",metavar="FILE",help="Python 코드로 변환")
    p.add_argument("file",nargs="?",help="실행할 .orange 파일"); a=p.parse_args()
    if a.translate:
        try: sys.stdout.write(translate(Path(a.translate).read_text(encoding="utf-8-sig"))); return 0
        except Exception as e: print(f"변환 오류: {e}",file=sys.stderr); return 1
    if not a.file: p.print_help(); return 2
    return run_file(Path(a.file))
if __name__=="__main__": raise SystemExit(main())
