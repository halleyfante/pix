export interface PixelColor {
  red: number;
  green: number;
  blue: number;
  alpha: number;
}

export class PixelInspector {
  private imageData: ImageData | null = null;
  private imageWidth: number = 0;
  private scale: number = 1;

  cacheImage(image: HTMLImageElement, scale: number): void {
    const canvas = document.createElement("canvas");
    canvas.width = image.width;
    canvas.height = image.height;
    const context = canvas.getContext("2d")!;
    context.drawImage(image, 0, 0);
    this.imageData = context.getImageData(0, 0, image.width, image.height);
    this.imageWidth = image.width;
    this.scale = scale;
  }

  getPixelColor(gridX: number, gridY: number): PixelColor | null {
    if (!this.imageData) {
      return null;
    }

    const sourceX = gridX * this.scale;
    const sourceY = gridY * this.scale;
    const index = (sourceY * this.imageWidth + sourceX) * 4;

    return {
      red: this.imageData.data[index],
      green: this.imageData.data[index + 1],
      blue: this.imageData.data[index + 2],
      alpha: this.imageData.data[index + 3],
    };
  }

  formatColor(pixel: PixelColor): string {
    if (pixel.alpha === 0) {
      return "transparent";
    }

    let hex = "#" +
      pixel.red.toString(16).padStart(2, "0") +
      pixel.green.toString(16).padStart(2, "0") +
      pixel.blue.toString(16).padStart(2, "0");

    if (pixel.alpha < 255) {
      hex += pixel.alpha.toString(16).padStart(2, "0");
    }

    return hex;
  }
}

export function formatPixelInfo(gridX: number, gridY: number, colorText: string): string {
  return "(" + gridX + ", " + gridY + ") " + colorText;
}
