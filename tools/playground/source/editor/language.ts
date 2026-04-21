import type * as Monaco from "monaco-editor";

export function registerPixLanguage(monaco: typeof Monaco): void {
  monaco.languages.register({
    id: "pix",
    extensions: [".pix"],
  });

  monaco.languages.setMonarchTokensProvider("pix", {
    keywords: [
      "grid", "by", "draw", "erase", "clear", "export",
      "in", "with", "to", "radius", "scale", "color",
      "not", "and", "or", "frame", "copy", "move", "at",
      "layer", "mirror",
    ],
    shapes: ["pixel", "line", "rectangle", "triangle", "circle"],
    formats: ["png", "svg", "webp", "gif"],
    operators: ["=", "<", ">", "<=", ">=", "+", "-", "*", "/", "^"],
    tokenizer: {
      root: [
        [/\/\/.*$/, "comment"],
        [/"[^"]*"/, "string"],
        [/#[0-9a-fA-F]{3,8}\b/, "number.hex"],
        [/\b[0-9]+\b/, "number"],
        [/\b(x|y)\b/, "variable"],
        [/\b[a-zA-Z]+\b/, {
          cases: {
            "@keywords": "keyword",
            "@shapes": "keyword",
            "@formats": "constant",
            "@default": "identifier",
          },
        }],
        [/[=<>+\-*/^]/, "operator"],
        [/[{}(),:]/, "delimiter"],
      ],
    },
  } as Monaco.languages.IMonarchLanguage);

  monaco.editor.defineTheme("pix-dark", {
    base: "vs-dark",
    inherit: false,
    rules: [
      { token: "comment", foreground: "706050" },
      { token: "string", foreground: "c4a882" },
      { token: "number", foreground: "d4a878" },
      { token: "number.hex", foreground: "d4a878" },
      { token: "keyword", foreground: "e84a00" },
      { token: "constant", foreground: "d4a878" },
      { token: "variable", foreground: "e8ddd0" },
      { token: "identifier", foreground: "b0a090" },
      { token: "operator", foreground: "b0a090" },
      { token: "delimiter", foreground: "b0a090" },
    ],
    colors: {
      "editor.background": "#1e1e1e",
      "editor.foreground": "#e8ddd0",
      "editor.lineHighlightBackground": "#2a2a2a",
      "editor.selectionBackground": "#333333",
      "editorCursor.foreground": "#e84a00",
      "editorLineNumber.foreground": "#706050",
      "editorLineNumber.activeForeground": "#b0a090",
    },
  });
}
