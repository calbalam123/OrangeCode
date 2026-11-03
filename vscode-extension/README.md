# Orange Code VS Code Extension 2.4.0

오렌지코드 파일의 문법 강조, 실행, 디버깅, Python 변환을 지원합니다.

## 요구 사항
- VS Code 1.80 이상
- Python 3.10 이상
- Microsoft Python 및 Python Debugger 확장

## 명령
- Run File: Ctrl+Shift+F5
- 디버깅 시작: F5
- Python 변환: Ctrl+Alt+T
- 새 파일 만들기: 명령 팔레트에서 실행

## 디버깅
1. Microsoft Python과 Python Debugger 확장을 설치합니다.
2. 저장된 .orange 파일을 열고 F5를 누릅니다.
3. 생성된 .orange.debug.py 파일에서 중단점을 설정합니다.
4. VS Code 디버그 도구 모음에서 계속, 한 단계씩 실행, 변수 확인을 할 수 있습니다.

디버거는 변환된 Python 파일을 대상으로 합니다. 중괄호 블록 닫는 줄을 빈 줄로 보존해 원본 줄 번호와 일치시키며, 중단점은 생성된 Python 파일에 설정해야 합니다.
