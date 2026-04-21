export interface ZipEntry {
  name: string;
  data: Uint8Array;
}

export function buildZip(files: ZipEntry[]): Uint8Array {
  const centralDirectoryEntries: Uint8Array[] = [];
  const fileEntries: Uint8Array[] = [];
  let offset = 0;

  for (const file of files) {
    const nameBytes = new TextEncoder().encode(file.name);
    const checksum = calculateCrc32(file.data);

    const localHeader = createLocalFileHeader(nameBytes, file.data, checksum);
    const centralHeader = createCentralDirectoryHeader(nameBytes, file.data, checksum, offset);

    fileEntries.push(localHeader);
    fileEntries.push(file.data);
    centralDirectoryEntries.push(centralHeader);

    offset += localHeader.byteLength + file.data.length;
  }

  const centralDirectorySize = centralDirectoryEntries.reduce(
    (sum, entry) => sum + entry.length,
    0,
  );

  const endRecord = createEndOfCentralDirectory(files.length, centralDirectorySize, offset);

  const totalSize = offset + centralDirectorySize + endRecord.byteLength;
  const result = new Uint8Array(totalSize);
  let position = 0;

  for (const entry of fileEntries) {
    result.set(entry, position);
    position += entry.length;
  }
  for (const entry of centralDirectoryEntries) {
    result.set(entry, position);
    position += entry.length;
  }
  result.set(new Uint8Array(endRecord), position);

  return result;
}

function createLocalFileHeader(
  nameBytes: Uint8Array,
  data: Uint8Array,
  checksum: number,
): Uint8Array {
  const header = new ArrayBuffer(30 + nameBytes.length);
  const view = new DataView(header);

  view.setUint32(0, 0x04034b50, true);
  view.setUint16(4, 20, true);
  view.setUint16(8, 0, true);
  view.setUint32(14, checksum, true);
  view.setUint32(18, data.length, true);
  view.setUint32(22, data.length, true);
  view.setUint16(26, nameBytes.length, true);

  new Uint8Array(header, 30).set(nameBytes);

  return new Uint8Array(header);
}

function createCentralDirectoryHeader(
  nameBytes: Uint8Array,
  data: Uint8Array,
  checksum: number,
  localHeaderOffset: number,
): Uint8Array {
  const header = new ArrayBuffer(46 + nameBytes.length);
  const view = new DataView(header);

  view.setUint32(0, 0x02014b50, true);
  view.setUint16(4, 20, true);
  view.setUint16(6, 20, true);
  view.setUint16(12, 0, true);
  view.setUint32(16, checksum, true);
  view.setUint32(20, data.length, true);
  view.setUint32(24, data.length, true);
  view.setUint16(28, nameBytes.length, true);
  view.setUint32(42, localHeaderOffset, true);

  new Uint8Array(header, 46).set(nameBytes);

  return new Uint8Array(header);
}

function createEndOfCentralDirectory(
  fileCount: number,
  centralDirectorySize: number,
  centralDirectoryOffset: number,
): ArrayBuffer {
  const record = new ArrayBuffer(22);
  const view = new DataView(record);

  view.setUint32(0, 0x06054b50, true);
  view.setUint16(8, fileCount, true);
  view.setUint16(10, fileCount, true);
  view.setUint32(12, centralDirectorySize, true);
  view.setUint32(16, centralDirectoryOffset, true);

  return record;
}

export function calculateCrc32(data: Uint8Array): number {
  let crc = 0xffffffff;
  for (let index = 0; index < data.length; index++) {
    crc ^= data[index];
    for (let bit = 0; bit < 8; bit++) {
      if (crc & 1) {
        crc = (crc >>> 1) ^ 0xedb88320;
      } else {
        crc = crc >>> 1;
      }
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}
