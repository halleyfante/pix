import { describe, it, expect, vi } from "vitest";

import { drawGridOverlay } from "../source/preview/grid-overlay.ts";

function createMockContext(): CanvasRenderingContext2D {
  return {
    strokeStyle: "",
    lineWidth: 0,
    beginPath: vi.fn(),
    moveTo: vi.fn(),
    lineTo: vi.fn(),
    stroke: vi.fn(),
  } as unknown as CanvasRenderingContext2D;
}

describe("drawGridOverlay", () => {
  it("draws vertical and horizontal lines", () => {
    const context = createMockContext();

    drawGridOverlay({
      context,
      gridWidth: 3,
      gridHeight: 2,
      cellSize: 10,
      canvasWidth: 30,
      canvasHeight: 20,
    });

    // 4 vertical lines (0, 1, 2, 3) + 3 horizontal lines (0, 1, 2) = 7 strokes
    expect(context.stroke).toHaveBeenCalledTimes(7);
  });

  it("sets stroke style to semi-transparent white", () => {
    const context = createMockContext();

    drawGridOverlay({
      context,
      gridWidth: 1,
      gridHeight: 1,
      cellSize: 10,
      canvasWidth: 10,
      canvasHeight: 10,
    });

    expect(context.strokeStyle).toBe("rgba(255, 255, 255, 0.2)");
  });

  it("draws lines at correct positions for cell size", () => {
    const context = createMockContext();

    drawGridOverlay({
      context,
      gridWidth: 2,
      gridHeight: 1,
      cellSize: 16,
      canvasWidth: 32,
      canvasHeight: 16,
    });

    // Vertical lines at x = 0.5, 16.5, 32.5
    expect(context.moveTo).toHaveBeenCalledWith(0.5, 0);
    expect(context.moveTo).toHaveBeenCalledWith(16.5, 0);
    expect(context.moveTo).toHaveBeenCalledWith(32.5, 0);
  });
});
