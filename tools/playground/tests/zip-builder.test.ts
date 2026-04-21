import { describe, it, expect } from "vitest";

import { buildZip, calculateCrc32 } from "../source/export/zip-builder.ts";

describe("calculateCrc32", () => {
  it("returns zero checksum for empty data", () => {
    const data = new Uint8Array(0);
    expect(calculateCrc32(data)).toBe(0);
  });

  it("returns correct checksum for known input", () => {
    const data = new TextEncoder().encode("hello");
    expect(calculateCrc32(data)).toBe(0x3610a686);
  });

  it("returns different checksums for different inputs", () => {
    const first = new TextEncoder().encode("abc");
    const second = new TextEncoder().encode("xyz");
    expect(calculateCrc32(first)).not.toBe(calculateCrc32(second));
  });
});

describe("buildZip", () => {
  it("creates a valid zip with local file header signature", () => {
    const data = new TextEncoder().encode("content");
    const zip = buildZip([{ name: "test.txt", data }]);
    const view = new DataView(zip.buffer);

    expect(view.getUint32(0, true)).toBe(0x04034b50);
  });

  it("creates a zip containing the file data", () => {
    const content = "hello world";
    const data = new TextEncoder().encode(content);
    const zip = buildZip([{ name: "greeting.txt", data }]);

    const zipString = new TextDecoder().decode(zip);
    expect(zipString).toContain("hello world");
    expect(zipString).toContain("greeting.txt");
  });

  it("creates a zip with multiple files", () => {
    const first = new TextEncoder().encode("first file");
    const second = new TextEncoder().encode("second file");
    const zip = buildZip([
      { name: "one.txt", data: first },
      { name: "two.txt", data: second },
    ]);

    const zipString = new TextDecoder().decode(zip);
    expect(zipString).toContain("one.txt");
    expect(zipString).toContain("two.txt");
    expect(zipString).toContain("first file");
    expect(zipString).toContain("second file");
  });

  it("includes end of central directory signature", () => {
    const data = new TextEncoder().encode("test");
    const zip = buildZip([{ name: "test.txt", data }]);
    const view = new DataView(zip.buffer);

    const endSignatureOffset = zip.length - 22;
    expect(view.getUint32(endSignatureOffset, true)).toBe(0x06054b50);
  });

  it("stores correct file count in end of central directory", () => {
    const data = new TextEncoder().encode("test");
    const zip = buildZip([
      { name: "a.txt", data },
      { name: "b.txt", data },
      { name: "c.txt", data },
    ]);
    const view = new DataView(zip.buffer);

    const endOffset = zip.length - 22;
    expect(view.getUint16(endOffset + 8, true)).toBe(3);
    expect(view.getUint16(endOffset + 10, true)).toBe(3);
  });

  it("stores correct compressed and uncompressed sizes", () => {
    const content = "exact content";
    const data = new TextEncoder().encode(content);
    const zip = buildZip([{ name: "file.txt", data }]);
    const view = new DataView(zip.buffer);

    expect(view.getUint32(18, true)).toBe(data.length);
    expect(view.getUint32(22, true)).toBe(data.length);
  });
});
