import { describe, it, expect } from "vitest";

import { PixelInspector, formatPixelInfo } from "../source/preview/pixel-inspector.ts";

describe("PixelInspector", () => {
  function createInspectorWithImageData(
    width: number,
    height: number,
    pixelData: number[],
    scale: number,
  ): PixelInspector {
    const inspector = new PixelInspector();
    const fakeImageData = {
      data: new Uint8ClampedArray(pixelData),
      width,
      height,
    };
    (inspector as unknown as { imageData: typeof fakeImageData }).imageData = fakeImageData;
    (inspector as unknown as { imageWidth: number }).imageWidth = width;
    (inspector as unknown as { scale: number }).scale = scale;
    return inspector;
  }

  it("returns null when no image is cached", () => {
    const inspector = new PixelInspector();
    expect(inspector.getPixelColor(0, 0)).toBeNull();
  });

  it("returns pixel color at grid position", () => {
    const inspector = createInspectorWithImageData(2, 2, [
      255, 0, 0, 255,
      0, 255, 0, 255,
      0, 0, 255, 255,
      255, 255, 0, 255,
    ], 1);

    const topLeft = inspector.getPixelColor(0, 0);
    expect(topLeft).toEqual({ red: 255, green: 0, blue: 0, alpha: 255 });

    const topRight = inspector.getPixelColor(1, 0);
    expect(topRight).toEqual({ red: 0, green: 255, blue: 0, alpha: 255 });

    const bottomLeft = inspector.getPixelColor(0, 1);
    expect(bottomLeft).toEqual({ red: 0, green: 0, blue: 255, alpha: 255 });
  });

  it("applies scale when reading pixel color", () => {
    const inspector = createInspectorWithImageData(4, 4, [
      // Row 0: red, red, green, green
      255, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
      // Row 1: red, red, green, green
      255, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255,
      // Row 2: blue, blue, yellow, yellow
      0, 0, 255, 255, 0, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 255,
      // Row 3: blue, blue, yellow, yellow
      0, 0, 255, 255, 0, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 255,
    ], 2);

    const topLeft = inspector.getPixelColor(0, 0);
    expect(topLeft).toEqual({ red: 255, green: 0, blue: 0, alpha: 255 });

    const topRight = inspector.getPixelColor(1, 0);
    expect(topRight).toEqual({ red: 0, green: 255, blue: 0, alpha: 255 });
  });

  it("formats opaque color as hex", () => {
    const inspector = new PixelInspector();
    const result = inspector.formatColor({ red: 232, green: 74, blue: 0, alpha: 255 });
    expect(result).toBe("#e84a00");
  });

  it("formats transparent color", () => {
    const inspector = new PixelInspector();
    const result = inspector.formatColor({ red: 0, green: 0, blue: 0, alpha: 0 });
    expect(result).toBe("transparent");
  });

  it("formats semi-transparent color with alpha", () => {
    const inspector = new PixelInspector();
    const result = inspector.formatColor({ red: 255, green: 0, blue: 0, alpha: 128 });
    expect(result).toBe("#ff000080");
  });
});

describe("formatPixelInfo", () => {
  it("formats coordinate and color", () => {
    expect(formatPixelInfo(5, 10, "#e84a00")).toBe("(5, 10) #e84a00");
  });

  it("formats with transparent", () => {
    expect(formatPixelInfo(0, 0, "transparent")).toBe("(0, 0) transparent");
  });
});
