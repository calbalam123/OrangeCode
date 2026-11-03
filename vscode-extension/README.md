# Orange Code VS Code Extension 2.3.0

오렌지코드(`.orange`) 파일의 문법 강조, 실행, Python 변환, 새 파일 만들기를 지원합니다.

## 요구 사항
- Visual Studio Code 1.80 이상
- Python 3.10 이상

## 명령
- **Orange Code: Run File** — 현재 `.orange` 파일 실행 (Ctrl+Shift+F5)
- **Orange Code: 파이썬 코드로 변환** — 현재 파일을 같은 폴더의 `.py` 파일로 변환 (Ctrl+Alt+T)
- **Orange Code: 새 파일 만들기** — 새 `.orange` 파일 생성

## VSIX 빌드
저장소 루트에서 GitHub Actions의 **Build VS Code extension** 워크플로를 실행하거나 다음 명령을 사용합니다.

```sh
cd vscode-extension
npm install
npx vsce package
```

설치: VS Code 명령 팔레트 → **Extensions: Install from VSIX...** → 생성된 VSIX 선택.
