# Orange Code 2.3.0

Python의 기능과 문법을 바탕으로 한국어 키워드·내장 함수 별칭을 제공하는 프로그래밍 언어입니다.

## 다운로드
- **전체 프로젝트 소스:** [OrangeCode-2.3.0-GitHub-Source.zip](https://github.com/calbalam123/OrangeCode/raw/refs/heads/main/OrangeCode-2.3.0-GitHub-Source.zip)
- **VS Code 확장:** [orange-code-2.3.0.vsix](https://github.com/calbalam123/OrangeCode/raw/refs/heads/main/orange-code-2.3.0.vsix)

## 설치
VS Code에서 `Ctrl+Shift+P` → `Extensions: Install from VSIX...` → 다운로드한 `.vsix` 파일을 선택하세요. Python 3.10 이상이 필요합니다.

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

## VS Code 명령
- Orange Code: Run File — 현재 `.orange` 파일 실행
- Orange Code: 파이썬 코드로 변환 — `.py` 파일 생성
- Orange Code: 새 파일 만들기 — 새 Orange Code 파일 생성

## 참고
Orange Code는 Python 변환 기반입니다. 한국어 별칭이 없는 Python 기능과 외부 패키지는 원래 Python 이름으로 계속 사용할 수 있습니다. 모든 외부 라이브러리 API가 한국어로 번역되는 것은 아닙니다.
