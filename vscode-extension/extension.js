const vscode = require("vscode");
const path = require("path");
const fs = require("fs");
const cp = require("child_process");

const quote = value => '"' + value.replace(/"/g, '\\"') + '"';
const runtimePath = context => path.join(context.extensionPath, "runtime", "orange.py");

function activate(context) {
  const run = vscode.commands.registerCommand("orange.runFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return vscode.window.showWarningMessage("먼저 .orange 파일을 열어 주세요.");
    const file = editor.document;
    if (file.isUntitled || !file.fileName.endsWith(".orange")) {
      return vscode.window.showWarningMessage("저장된 .orange 파일에서 실행해 주세요.");
    }
    await file.save();
    const runtime = runtimePath(context);
    if (!fs.existsSync(runtime)) return vscode.window.showErrorMessage("Orange Code 실행기를 찾을 수 없습니다.");
    const terminal = vscode.window.createTerminal({ name: "Orange Code" });
    const py = process.platform === "win32" ? "py -3" : "python3";
    terminal.show(true);
    terminal.sendText(`${py} ${quote(runtime)} ${quote(file.fileName)}`);
  });

  const translate = vscode.commands.registerCommand("orange.translateFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.isUntitled || !editor.document.fileName.endsWith(".orange")) {
      return vscode.window.showWarningMessage("저장된 .orange 파일을 먼저 열어 주세요.");
    }
    await editor.document.save();
    const runtime = runtimePath(context);
    const py = process.platform === "win32" ? "py" : "python3";
    const args = process.platform === "win32"
      ? ["-3", runtime, "--translate", editor.document.fileName]
      : [runtime, "--translate", editor.document.fileName];
    cp.execFile(py, args, { encoding: "utf8" }, (error, stdout, stderr) => {
      if (error) return vscode.window.showErrorMessage(stderr || error.message);
      const out = editor.document.fileName.replace(/\.orange$/i, ".py");
      fs.writeFileSync(out, stdout, "utf8");
      vscode.window.showInformationMessage(`Python 파일로 변환했습니다: ${path.basename(out)}`);
      vscode.workspace.openTextDocument(out).then(doc => vscode.window.showTextDocument(doc));
    });
  });

  const create = vscode.commands.registerCommand("orange.createFile", async () => {
    const uri = await vscode.window.showSaveDialog({
      filters: { "Orange Code": ["orange"] },
      saveLabel: "Orange Code 파일 만들기"
    });
    if (!uri) return;
    fs.writeFileSync(uri.fsPath, '# Orange Code 2.3.0\n출력("안녕하세요, 오렌지코드!")\n', "utf8");
    const doc = await vscode.workspace.openTextDocument(uri);
    await vscode.window.showTextDocument(doc);
  });

  context.subscriptions.push(run, translate, create);
}

function deactivate() {}
module.exports = { activate, deactivate };
