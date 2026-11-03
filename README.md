# 오렌지코드 (Orange Code)

한국어 명령어로 작성하고 실행하는 초보자용 프로그래밍 언어입니다.

## 주요 기능
- 한국어 변수 선언: `변수 이름 = 값`
- 출력과 입력: `출력하기(...)`, `입력하기(...)`
- 조건문: `만약 조건 라면 {` / `아니면 {`
- 반복문: `반복하기 3번 {`, `동안 조건 반복하기 {`
- 목록 반복: `각각 항목 안에 목록 반복하기 {`
- 함수와 반환: `함수 인사(이름) {`, `반환 값`
- VS Code 문법 강조, 스니펫, 실행 명령

## 준비물
- Windows 10/11
- Python 3.10 이상
- Visual Studio Code

## 빠른 실행
1. Python 3.10 이상을 설치하고 설치 화면에서 **Add Python to PATH**를 선택합니다.
2. 이 저장소를 내려받거나 Git으로 복제합니다.
3. 터미널에서 프로젝트 폴더로 이동합니다.
4. 다음 명령으로 예제를 실행합니다.

```powershell
py -3 runtime/orange.py examples/hello.orange
```

Python 명령이 `py -3`가 아니라면 `python` 또는 `python3`를 사용하세요.

## VS Code 확장 설치
저장소의 `vscode-extension` 폴더를 VS Code에서 열고 터미널에서 실행합니다.

```powershell
cd vscode-extension
npm install
npm install --global @vscode/vsce
vsce package
```

생성된 `.vsix` 파일을 VS Code에서 설치합니다.
- `Ctrl + Shift + X`를 누릅니다.
- 확장 메뉴의 `...`를 누릅니다.
- **Install from VSIX...**를 선택합니다.
- 생성된 `orange-code-1.0.0.vsix`를 선택합니다.

## 예제
```orange
변수 이름 = "오렌지"
출력하기("안녕, " + 이름)

변수 점수 = 85
만약 점수 >= 80 라면 {
    출력하기("합격!")
} 아니면 {
    출력하기("다시 도전!")
}

반복하기 3번 {
    출력하기("오렌지코드")
}
```

## VS Code에서 실행
`.orange` 파일을 열고 `Ctrl + Shift + P` → **Orange Code: Run File**을 실행합니다.

## 문법 참고
- 참/거짓 값은 `참`, `거짓`입니다.
- 논리 연산자는 `그리고`, `또는`, `아니다`를 사용할 수 있습니다.
- 블록은 중괄호 `{`와 `}`로 감쌉니다.
- 한 줄에 하나의 명령문을 작성하는 방식을 권장합니다.

## 현재 버전의 범위
Orange Code 1.0.0은 한국어 문법을 Python으로 변환해 실행하는 교육용 인터프리터입니다. Python 3.10 이상이 필요합니다. Python과 완전히 독립된 네이티브 언어는 아니며, 임의의 시스템 명령 실행이나 샌드박스 보안을 보장하지 않습니다.

## 라이선스
MIT
