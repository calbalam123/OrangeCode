# Orange Code 2.4.1

Python의 문법과 표준 기능을 기반으로 한국어 키워드와 내장 함수 별칭을 제공하는 프로그래밍 언어입니다.

## VS Code 확장 다운로드
1. [GitHub Actions 빌드 페이지](https://github.com/calbalam123/OrangeCode/actions/workflows/build-vsix.yml)를 엽니다.
2. 가장 최근의 **Build VS Code extension** 실행을 선택합니다.
3. 페이지 아래 **Artifacts**에서 `orange-code-vsix`를 다운로드합니다.
4. 압축을 풀고 VS Code에서 `Ctrl+Shift+P` → `Extensions: Install from VSIX...` → `.vsix` 선택.

## 한국어 인코딩
`.orange` 파일은 UTF-8로 저장하세요. VS Code 확장은 UTF-8 편집·실행 환경을 기본 적용해 Windows에서도 한글 입출력을 지원합니다.

## 준비물
- Python 3.10 이상
- VS Code 1.80 이상 (확장 기능 사용 시)

## 빠른 실행
Windows:
```powershell
py -3 runtime/orange.py examples/hello.orange
```

macOS/Linux:
```sh
python3 runtime/orange.py examples/hello.orange
```

## 예제
```python
이름 = 입력("이름을 입력하세요: ")
만약 이름 == "설이":
    출력("환영합니다!")
아니면:
    출력("안녕하세요", 이름)

함수 제곱(숫자):
    반환 숫자 ** 2

반복 숫자 안에 범위(5):
    출력(제곱(숫자))
```

## 주요 한국어 문법
| Python | Orange Code |
|---|---|
| if / elif / else | 만약 / 아니면만약 / 아니면 |
| for / while | 반복 / 동안 |
| def / class / return | 함수 / 클래스 / 반환 |
| try / except / finally | 시도 / 예외 / 마지막으로 |
| import / from / as | 가져오기 / 에서 / 별칭 |
| True / False / None | 참 / 거짓 / 없음 |
| and / or / not / in / is | 그리고 / 또는 / 아니다 / 안에 / 같은가 |
| break / continue / pass | 중단 / 계속 / 아무것도안함 |

출력, 입력, 길이, 정수, 실수, 문자열, 목록, 튜플, 집합, 사전, 범위, 열거, 압축, 지도, 필터, 정렬, 역순, 합계, 최대, 최소, 절댓값, 반올림, 거듭제곱, 모든참, 하나라도참, 자료형, 도움말, 열기 등의 한국어 별칭을 지원합니다.

## VS Code 명령
- **Orange Code: Run File** — `.orange` 파일 실행
- **Orange Code: 디버깅 시작** — F5로 변환된 Python 코드를 디버거에서 실행 (Python 및 Python Debugger 확장 필요)
- **Orange Code: 파이썬 코드로 변환** — 같은 폴더에 `.py` 생성
- **Orange Code: 새 파일 만들기** — 새 Orange Code 파일 생성

## 범위
Orange Code는 Python 변환 기반 언어입니다. 별칭이 없는 Python 기능과 외부 패키지는 원래 Python 이름으로 사용할 수 있습니다. 모든 외부 라이브러리 API가 한국어로 번역되는 것은 아닙니다.
