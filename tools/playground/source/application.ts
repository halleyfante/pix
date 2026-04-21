import * as monaco from "monaco-editor";

import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";

import { registerPixLanguage } from "./editor/language.ts";
import { registerCompletionProvider } from "./editor/completion.ts";
import { createEditor, getEditorValue } from "./editor/editor.ts";
import { renderSource } from "./client/render-client.ts";
import { Preview } from "./preview/preview.ts";

self.MonacoEnvironment = {
  getWorker(): Worker {
    return new editorWorker();
  },
};

registerPixLanguage(monaco);
registerCompletionProvider(monaco);

const editorContainer = document.getElementById("editor")!;
const runButton = document.getElementById("run-button") as HTMLButtonElement;
const preview = new Preview();

const editor = createEditor(monaco, editorContainer);

async function run(): Promise<void> {
  runButton.disabled = true;

  const source = getEditorValue();

  try {
    const blob = await renderSource(source);
    preview.load(blob, source);
  } catch (error) {
    if (error instanceof Error) {
      preview.showError(error.message);
    }
  } finally {
    runButton.disabled = false;
  }
}

runButton.addEventListener("click", run);

editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, run);

window.addEventListener("resize", () => {
  editor.layout();
});
