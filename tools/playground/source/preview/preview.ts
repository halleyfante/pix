import { drawGridOverlay } from "./grid-overlay.ts";
import { PixelInspector, formatPixelInfo } from "./pixel-inspector.ts";
import { exportProject } from "../export/exporter.ts";

export class Preview {
  private canvas: HTMLCanvasElement;
  private context: CanvasRenderingContext2D;
  private message: HTMLElement;
  private gridSizeLabel: HTMLElement;
  private pixelInfoLabel: HTMLElement;
  private gridToggleButton: HTMLElement;
  private exportButton: HTMLButtonElement;

  private image: HTMLImageElement | null = null;
  private imageBlob: Blob | null = null;
  private gridWidth: number = 0;
  private gridHeight: number = 0;
  private scale: number = 1;
  private zoom: number = 1;
  private showGrid: boolean = true;
  private sourceCode: string = "";
  private hoveredGridX: number = -1;
  private hoveredGridY: number = -1;
  private pixelInspector: PixelInspector = new PixelInspector();

  constructor() {
    this.canvas = document.getElementById("preview-canvas") as HTMLCanvasElement;
    this.context = this.canvas.getContext("2d")!;
    this.message = document.getElementById("preview-message")!;
    this.gridSizeLabel = document.getElementById("grid-size")!;
    this.pixelInfoLabel = document.getElementById("pixel-info")!;
    this.gridToggleButton = document.getElementById("grid-toggle")!;
    this.exportButton = document.getElementById("export-button") as HTMLButtonElement;

    const viewport = document.getElementById("preview-viewport")!;

    this.gridToggleButton.addEventListener("click", () => {
      this.showGrid = !this.showGrid;
      this.gridToggleButton.classList.toggle("active", this.showGrid);
      this.draw();
    });

    viewport.addEventListener("wheel", (event: WheelEvent) => {
      event.preventDefault();
      if (event.deltaY < 0) {
        this.zoom = Math.min(this.zoom * 1.2, 32);
      } else {
        this.zoom = Math.max(this.zoom / 1.2, 0.5);
      }
      this.draw();
    });

    this.canvas.addEventListener("mousemove", (event: MouseEvent) => {
      this.handleHover(event);
    });

    this.canvas.addEventListener("mouseleave", () => {
      this.hoveredGridX = -1;
      this.hoveredGridY = -1;
      this.pixelInfoLabel.textContent = "";
      this.draw();
    });

    this.exportButton.addEventListener("click", () => {
      if (this.imageBlob) {
        exportProject(this.imageBlob, this.sourceCode);
      }
    });
  }

  load(blob: Blob, sourceCode: string): void {
    this.imageBlob = blob;
    this.sourceCode = sourceCode;
    this.image = new Image();
    this.image.onload = () => {
      this.parseGridDimensions();
      this.pixelInspector.cacheImage(this.image!, this.scale);
      this.zoom = 1;
      this.message.style.display = "none";
      this.canvas.style.display = "block";
      this.exportButton.disabled = false;
      this.draw();
    };
    this.image.src = URL.createObjectURL(blob);
  }

  showError(text: string): void {
    this.canvas.style.display = "none";
    this.message.style.display = "block";
    this.message.textContent = text;
    this.message.classList.add("preview-error");
    this.exportButton.disabled = true;
  }

  private parseGridDimensions(): void {
    const match = this.sourceCode.match(/grid\s+(\d+)\s+by\s+(\d+)/);
    if (match) {
      this.gridWidth = parseInt(match[1]);
      this.gridHeight = parseInt(match[2]);
    } else if (this.image) {
      this.gridWidth = this.image.width;
      this.gridHeight = this.image.height;
    }

    const scaleMatch = this.sourceCode.match(/scale\s+(\d+)/);
    this.scale = scaleMatch ? parseInt(scaleMatch[1]) : 1;

    this.gridSizeLabel.textContent = this.gridWidth + " x " + this.gridHeight;
  }

  private draw(): void {
    if (!this.image) {
      return;
    }

    const cellSize = this.scale * this.zoom;
    const canvasWidth = this.gridWidth * cellSize;
    const canvasHeight = this.gridHeight * cellSize;

    this.canvas.width = canvasWidth;
    this.canvas.height = canvasHeight;
    this.canvas.style.width = canvasWidth + "px";
    this.canvas.style.height = canvasHeight + "px";

    this.context.imageSmoothingEnabled = false;
    this.context.drawImage(this.image, 0, 0, canvasWidth, canvasHeight);

    if (this.showGrid && cellSize >= 4) {
      drawGridOverlay({
        context: this.context,
        gridWidth: this.gridWidth,
        gridHeight: this.gridHeight,
        cellSize,
        canvasWidth,
        canvasHeight,
      });
    }

    if (this.hoveredGridX >= 0 && this.hoveredGridY >= 0) {
      this.drawHighlight(cellSize);
    }
  }

  private drawHighlight(cellSize: number): void {
    const x = this.hoveredGridX * cellSize;
    const y = this.hoveredGridY * cellSize;

    this.context.strokeStyle = "rgba(0, 0, 0, 0.6)";
    this.context.lineWidth = 3;
    this.context.strokeRect(x + 1.5, y + 1.5, cellSize - 3, cellSize - 3);

    this.context.strokeStyle = "rgba(255, 255, 255, 0.9)";
    this.context.lineWidth = 1;
    this.context.strokeRect(x + 1.5, y + 1.5, cellSize - 3, cellSize - 3);
  }

  private handleHover(event: MouseEvent): void {
    if (!this.image || this.gridWidth === 0) {
      return;
    }

    const rect = this.canvas.getBoundingClientRect();
    const mouseX = event.clientX - rect.left;
    const mouseY = event.clientY - rect.top;

    const cellSize = this.scale * this.zoom;
    const gridX = Math.floor(mouseX / cellSize);
    const gridY = Math.floor(mouseY / cellSize);

    if (gridX < 0 || gridX >= this.gridWidth || gridY < 0 || gridY >= this.gridHeight) {
      this.hoveredGridX = -1;
      this.hoveredGridY = -1;
      this.pixelInfoLabel.textContent = "";
      this.draw();
      return;
    }

    this.hoveredGridX = gridX;
    this.hoveredGridY = gridY;

    const pixel = this.pixelInspector.getPixelColor(gridX, gridY);
    if (pixel) {
      const colorText = this.pixelInspector.formatColor(pixel);
      this.pixelInfoLabel.textContent = formatPixelInfo(gridX, gridY, colorText);
    }

    this.draw();
  }
}
