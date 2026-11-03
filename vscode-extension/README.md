# Orange Code VS Code Extension 2.4.1

오렌지코드(.orange)의 한국어 문법 강조, 실행, 디버깅, Python 변환을 지원합니다.

## 요구 사항
- VS Code 1.80 이상
- Python 3.10 이상
- Microsoft Python 및 Python Debugger 확장(디버깅 시)

## 한국어 인코딩
- .orange 파일은 UTF-8로 저장합니다. VS Code 작업공간은 UTF-8을 기본 인코딩으로 사용합니다.
- 실행 및 변환 과정에서 Python UTF-8 모드와 UTF-8 표준 입출력을 사용합니다.
- Windows에서도 한글 문자열, 주석, 입력 및 출력이 깨지지 않도록 실행 환경을 설정합니다.

## 명령
- **Orange Code: Run File** — 현재 .orange 파일 실행 (Ctrl+Shift+F5)
- **Orange Code: 디버깅 시작** — F5로 변환된 Python 코드 디버깅
- **Orange Code: 파이썬 코드로 변환** — 같은 폴더에 .py 파일 생성 (Ctrl+Alt+T)
- **Orange Code: 새 파일 만들기** — 새 .orange 파일 생성

## 디버깅
1. Microsoft Python 및 Python Debugger 확장을 설치합니다.
2. 저장된 .orange 파일을 열고 F5를 누릅니다.
3. 생성된 .orange.debug.py 파일에서 중단점을 설정합니다.
4. VS Code 디버그 도구 모음에서 계속, 한 단계씩 실행, 변수 확인을 할 수 있습니다.

디버거는 변환된 Python 파일을 대상으로 합니다. 중괄호 블록의 닫는 줄은 빈 줄로 보존하여 원본 줄 번호를 맞춥니다.
