import { describe, it, expect, vi, beforeEach } from "vitest";

import { renderSource } from "../source/client/render-client.ts";

describe("renderSource", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("sends POST request with source code", async () => {
    const mockBlob = new Blob(["image data"], { type: "image/png" });
    const mockResponse = new Response(mockBlob, { status: 200 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    await renderSource("grid 5 by 5");

    expect(fetch).toHaveBeenCalledWith("/render", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ source: "grid 5 by 5" }),
    });
  });

  it("returns blob on success", async () => {
    const mockBlob = new Blob(["image data"], { type: "image/png" });
    const mockResponse = new Response(mockBlob, { status: 200 });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    const result = await renderSource("grid 5 by 5");
    expect(result).toBeInstanceOf(Blob);
  });

  it("throws error on failure", async () => {
    const errorBody = JSON.stringify({ error: "missing grid statement" });
    const mockResponse = new Response(errorBody, {
      status: 422,
      headers: { "Content-Type": "application/json" },
    });
    vi.spyOn(globalThis, "fetch").mockResolvedValue(mockResponse);

    await expect(renderSource("invalid")).rejects.toThrow("missing grid statement");
  });
});
