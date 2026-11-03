const vscode = require("vscode");
const path = require("path");
const fs = require("fs");
const cp = require("child_process");

function activate(context) {
  const disposable = vscode.commands.registerCommand("orange.runFile", () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showWarningMessage("먼저 .orange 파일을 열어 주세요.");
      return;
    }
    const file = editor.document;
    if (file.isUntitled || !file.fileName.endsWith(".orange")) {
      vscode.window.showWarningMessage("저장된 .orange 파일에서 실행해 주세요.");
      return;
    }
    file.save().then(() => {
      const extensionRoot = context.extensionPath;
      const runtime = path.join(extensionRoot, "runtime", "orange.py");
      if (!fs.existsSync(runtime)) {
        vscode.window.showErrorMessage("오렌지코드 실행기 파일을 찾을 수 없습니다.");
        return;
      }
      const terminal = vscode.window.createTerminal({ name: "Orange Code" });
      const quoted = value => '"' + value.replace(/"/g, '\\"') + '"';
      const isWindows = process.platform === "win32";
      const command = isWindows
        ? "py -3 " + quoted(runtime) + " " + quoted(file.fileName)
        : "python3 " + quoted(runtime) + " " + quoted(file.fileName);
      terminal.show(true);
      terminal.sendText(command);
    });
  });
  context.subscriptions.push(disposable);
}

function deactivate() {}
module.exports = { activate, deactivate };
