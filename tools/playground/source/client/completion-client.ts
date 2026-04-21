export enum CompletionKind {
  Keyword = "Keyword",
  Color = "Color",
  Variable = "Variable",
  Format = "Format",
  Snippet = "Snippet",
}

export interface CompletionItem {
  label: string;
  kind: CompletionKind;
  snippet?: string;
}

export async function fetchCompletions(
  source: string,
  line: number,
  column: number,
): Promise<CompletionItem[]> {
  const response = await fetch("/complete", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ source, line, column }),
  });

  if (!response.ok) {
    return [];
  }

  return response.json();
}
