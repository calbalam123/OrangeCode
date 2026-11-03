# Orange Code VS Code Extension

오렌지코드(`.orange`) 파일의 문법 강조와 네이티브 실행 기능을 제공합니다.

## Requirements
- Visual Studio Code 1.80+
- Orange Code Native 2.0 실행 파일

## Configure the runtime
1. Rust로 `native` 프로젝트를 빌드해 `orange.exe`를 만듭니다.
2. 실행 파일을 PATH에 등록하거나 VS Code 설정에서 `orange.executablePath`에 전체 경로를 입력합니다.
3. 저장된 `.orange` 파일에서 `Ctrl+Shift+F5`를 누르거나 **Orange Code: Run File** 명령을 실행합니다.

Python 설치는 필요하지 않습니다.

## Build VSIX
From this directory:

```sh
npm install
npx vsce package
```

Install the resulting `.vsix` from VS Code Extensions view → `...` → **Install from VSIX...**.

## Legacy runtime
The old Python runtime remains in the repository for Orange Code 1.x compatibility. The 2.0 extension does not invoke it.
