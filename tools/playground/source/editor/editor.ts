import type * as Monaco from "monaco-editor";

const DEFAULT_CODE = `grid 17 by 17

color {
    warm: #e84a00
}

circle (5, 5) radius 4 with color warm
circle (11, 5) radius 4 with color warm
triangle (1, 7) to (15, 7) to (8, 15) with color warm

export "heart" in png scale 8`;

let editorInstance: Monaco.editor.IStandaloneCodeEditor;

export function createEditor(
  monaco: typeof Monaco,
  container: HTMLElement,
): Monaco.editor.IStandaloneCodeEditor {
  editorInstance = monaco.editor.create(container, {
    value: DEFAULT_CODE,
    language: "pix",
    theme: "pix-dark",
    fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
    fontSize: 14,
    lineHeight: 1.75,
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    padding: { top: 16 },
    renderLineHighlight: "line",
    overviewRulerLanes: 0,
    hideCursorInOverviewRuler: true,
    overviewRulerBorder: false,
    scrollbar: {
      verticalScrollbarSize: 8,
      horizontalScrollbarSize: 8,
    },
  });

  return editorInstance;
}

export function getEditorValue(): string {
  return editorInstance.getValue();
}
