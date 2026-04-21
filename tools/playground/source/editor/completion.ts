import type * as Monaco from "monaco-editor";

import { CompletionKind, fetchCompletions } from "../client/completion-client.ts";

const COMPLETION_KIND_MAP: Record<string, number> = {
  [CompletionKind.Keyword]: 14,
  [CompletionKind.Color]: 16,
  [CompletionKind.Variable]: 6,
  [CompletionKind.Format]: 21,
  [CompletionKind.Snippet]: 15,
};

export function registerCompletionProvider(monaco: typeof Monaco): void {
  monaco.languages.registerCompletionItemProvider("pix", {
    triggerCharacters: [" "],
    provideCompletionItems: async (
      model: Monaco.editor.ITextModel,
      position: Monaco.Position,
    ): Promise<Monaco.languages.CompletionList> => {
      const source = model.getValue();
      const wordInfo = model.getWordUntilPosition(position);
      const range: Monaco.IRange = {
        startLineNumber: position.lineNumber,
        startColumn: wordInfo.startColumn,
        endLineNumber: position.lineNumber,
        endColumn: position.column,
      };

      const items = await fetchCompletions(
        source,
        position.lineNumber,
        position.column,
      );

      return {
        suggestions: items.map((item) => {
          const suggestion: Monaco.languages.CompletionItem = {
            label: item.label,
            kind: COMPLETION_KIND_MAP[item.kind] || monaco.languages.CompletionItemKind.Text,
            insertText: item.snippet || item.label,
            range,
          };
          if (item.snippet) {
            suggestion.insertTextRules = monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet;
          }
          return suggestion;
        }),
      };
    },
  });
}
