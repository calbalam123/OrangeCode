const vscode = require("vscode");
const path = require("path");
const fs = require("fs");
const cp = require("child_process");

function quote(value) { return '"' + value.replace(/"/g, '\\"') + '"'; }
function runtimePath(context) { return path.join(context.extensionPath, "runtime", "orange.py"); }

function activate(context) {
  const run = vscode.commands.registerCommand("orange.runFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return vscode.window.showWarningMessage("먼저 .orange 파일을 열어 주세요.");
    const file = editor.document;
    if (file.isUntitled || !file.fileName.endsWith(".orange")) return vscode.window.showWarningMessage("저장된 .orange 파일에서 실행해 주세요.");
    await file.save();
    const runtime = runtimePath(context);
    if (!fs.existsSync(runtime)) return vscode.window.showErrorMessage("Orange Code 실행기를 찾을 수 없습니다.");
    const terminal = vscode.window.createTerminal({ name: "Orange Code", env: { PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8" } });
    const py = process.platform === "win32" ? "py -3 -X utf8" : "python3 -X utf8";
    terminal.show(true);
    terminal.sendText(py + " " + quote(runtime) + " " + quote(file.fileName));
  });

  const debug = vscode.commands.registerCommand("orange.debugFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.isUntitled || !editor.document.fileName.endsWith(".orange")) return vscode.window.showWarningMessage("저장된 .orange 파일을 먼저 열어 주세요.");
    await editor.document.save();
    if (!vscode.extensions.getExtension("ms-python.debugpy")) {
      const choice = await vscode.window.showWarningMessage("디버깅하려면 Microsoft Python Debugger 확장을 설치해야 합니다.", "확장 설치 안내");
      if (choice === "확장 설치 안내") await vscode.env.openExternal(vscode.Uri.parse("https://marketplace.visualstudio.com/items?itemName=ms-python.debugpy"));
      return;
    }
    const runtime = runtimePath(context);
    if (!fs.existsSync(runtime)) return vscode.window.showErrorMessage("Orange Code 실행기를 찾을 수 없습니다.");
    const sourceFile = editor.document.fileName;
    const debugFile = sourceFile.replace(/\.orange$/i, ".orange.debug.py");
    const python = process.platform === "win32" ? "py" : "python3";
    const args = process.platform === "win32" ? ["-3", "-X", "utf8", runtime, "--translate", sourceFile] : ["-X", "utf8", runtime, "--translate", sourceFile];
    cp.execFile(python, args, { encoding: "utf8", env: { ...process.env, PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8" } }, async (error, stdout, stderr) => {
      if (error) return vscode.window.showErrorMessage(stderr || error.message);
      try {
        fs.writeFileSync(debugFile, stdout, "utf8");
        const debugDoc = await vscode.workspace.openTextDocument(debugFile);
        await vscode.window.showTextDocument(debugDoc, { preview: false });
        const folder = vscode.workspace.getWorkspaceFolder(editor.document.uri);
        const started = await vscode.debug.startDebugging(folder, { name:"Orange Code 디버깅", type:"debugpy", request:"launch", program:debugFile, console:"integratedTerminal", cwd:path.dirname(sourceFile), justMyCode:true });
        if (!started) vscode.window.showErrorMessage("디버거를 시작하지 못했습니다. Python Debugger 확장을 확인해 주세요.");
      } catch (e) { vscode.window.showErrorMessage("디버깅 준비 실패: " + e.message); }
    });
  });

  const translate = vscode.commands.registerCommand("orange.translateFile", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.isUntitled || !editor.document.fileName.endsWith(".orange")) return vscode.window.showWarningMessage("저장된 .orange 파일을 먼저 열어 주세요.");
    await editor.document.save();
    const runtime = runtimePath(context);
    const py = process.platform === "win32" ? "py" : "python3";
    const args = process.platform === "win32" ? ["-3","-X","utf8",runtime,"--translate",editor.document.fileName] : ["-X","utf8",runtime,"--translate",editor.document.fileName];
    cp.execFile(py,args,{encoding:"utf8",env:{...process.env,PYTHONUTF8:"1",PYTHONIOENCODING:"utf-8"}},(error,stdout,stderr)=>{
      if(error) return vscode.window.showErrorMessage(stderr || error.message);
      const out = editor.document.fileName.replace(/\.orange$/i,".py");
      fs.writeFileSync(out,stdout,"utf8");
      vscode.window.showInformationMessage("파이썬 파일로 변환했습니다: " + path.basename(out));
      vscode.workspace.openTextDocument(out).then(doc=>vscode.window.showTextDocument(doc));
    });
  });

  const create = vscode.commands.registerCommand("orange.createFile", async () => {
    const uri = await vscode.window.showSaveDialog({filters:{"Orange Code":["orange"]},saveLabel:"Orange Code 파일 만들기"});
    if(!uri) return;
    fs.writeFileSync(uri.fsPath, '# Orange Code 2.4.0\n출력("안녕하세요, 오렌지코드!")\n',"utf8");
    const doc = await vscode.workspace.openTextDocument(uri);
    await vscode.window.showTextDocument(doc);
  });
  context.subscriptions.push(run,debug,translate,create);
}
function deactivate() {}
module.exports = { activate, deactivate };