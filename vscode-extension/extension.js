const vscode = require("vscode");

function quote(value) {
  return '"' + value.replace(/"/g, '\\"') + '"';
}

function activate(context) {
  const disposable = vscode.commands.registerCommand("orange.runFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showWarningMessage("먼저 .orange 파일을 열어 주세요.");
      return;
    }
    const file = editor.document;
    if (file.isUntitled || !file.fileName.toLowerCase().endsWith(".orange")) {
      vscode.window.showWarningMessage("저장된 .orange 파일에서 실행해 주세요.");
      return;
    }
    await file.save();

    const config = vscode.workspace.getConfiguration("orange");
    const executable = config.get("executablePath", "orange");
    const terminal = vscode.window.createTerminal({ name: "Orange Code Native" });
    const isWindows = process.platform === "win32";
    const command = isWindows
      ? "& " + quote(executable) + " " + quote(file.fileName)
      : quote(executable) + " " + quote(file.fileName);
    terminal.show(true);
    terminal.sendText(command);
  });
  context.subscriptions.push(disposable);
}

function deactivate() {}
module.exports = { activate, deactivate };
