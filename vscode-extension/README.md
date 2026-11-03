# Orange Code VS Code Extension

오렌지코드(`.orange`) 파일의 문법 강조와 실행 기능을 제공합니다.

## Requirements
- Visual Studio Code 1.80+
- Python 3.10+

## Run
Open a saved `.orange` file and press `Ctrl+Shift+F5`, or run **Orange Code: Run File** from the Command Palette.

The extension bundles the Orange Code Python runtime in its `runtime` directory.

## Build VSIX
From this directory:

```sh
npm install
npx vsce package
```

Install the resulting `.vsix` from VS Code Extensions view → `...` → **Install from VSIX...**.
