import { describe, it, expect, vi, beforeEach } from "vitest";

import { fetchCompletions, CompletionKind } from "../source/client/completion-client.ts";

describe("fetchCompletions", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("sends POST request with source, line, and column", async () => {
    const mockItems = [{ label: "grid", kind: CompletionKind.Keyword }];
    const mockResponse = new Response(JSON.stringify(mockItems), { status: 200 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    await fetchCompletions("gr", 1, 3);

    expect(fetch).toHaveBeenCalledWith("/complete", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ source: "gr", line: 1, column: 3 }),
    });
  });

  it("returns completion items on success", async () => {
    const mockItems = [
      { label: "grid", kind: CompletionKind.Keyword },
      { label: "draw", kind: CompletionKind.Keyword },
    ];
    const mockResponse = new Response(JSON.stringify(mockItems), { status: 200 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    const result = await fetchCompletions("", 1, 1);
    expect(result).toHaveLength(2);
    expect(result[0].label).toBe("grid");
    expect(result[1].label).toBe("draw");
  });

  it("returns empty array on failure", async () => {
    const mockResponse = new Response("error", { status: 500 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    const result = await fetchCompletions("", 1, 1);
    expect(result).toEqual([]);
  });

  it("returns items with snippets", async () => {
    const mockItems = [{
      label: "(x, y)",
      kind: CompletionKind.Snippet,
      snippet: "(${1}, ${2})",
    }];
    const mockResponse = new Response(JSON.stringify(mockItems), { status: 200 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    const result = await fetchCompletions("circle ", 1, 8);
    expect(result[0].snippet).toBe("(${1}, ${2})");
  });
});
