import { buildZip } from "./zip-builder.ts";

export async function exportProject(imageBlob: Blob, sourceCode: string): Promise<void> {
  const exportMatch = sourceCode.match(/export\s+"([^"]+)"\s+in\s+(\w+)/);
  const filename = exportMatch ? exportMatch[1] : "image";
  const format = exportMatch ? exportMatch[2] : "png";

  const imageBytes = new Uint8Array(await imageBlob.arrayBuffer());
  const sourceBytes = new TextEncoder().encode(sourceCode);

  const zipData = buildZip([
    { name: filename + ".pix", data: sourceBytes },
    { name: filename + "." + format, data: imageBytes },
  ]);

  const blob = new Blob([zipData], { type: "application/zip" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename + ".zip";
  link.click();
  URL.revokeObjectURL(url);
}
