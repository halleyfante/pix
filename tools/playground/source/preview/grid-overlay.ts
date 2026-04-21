export interface GridOverlayOptions {
  context: CanvasRenderingContext2D;
  gridWidth: number;
  gridHeight: number;
  cellSize: number;
  canvasWidth: number;
  canvasHeight: number;
}

export function drawGridOverlay(options: GridOverlayOptions): void {
  const { context, gridWidth, gridHeight, cellSize, canvasWidth, canvasHeight } = options;

  context.strokeStyle = "rgba(255, 255, 255, 0.2)";
  context.lineWidth = 1;

  for (let x = 0; x <= gridWidth; x++) {
    const pixelX = Math.floor(x * cellSize) + 0.5;
    context.beginPath();
    context.moveTo(pixelX, 0);
    context.lineTo(pixelX, canvasHeight);
    context.stroke();
  }

  for (let y = 0; y <= gridHeight; y++) {
    const pixelY = Math.floor(y * cellSize) + 0.5;
    context.beginPath();
    context.moveTo(0, pixelY);
    context.lineTo(canvasWidth, pixelY);
    context.stroke();
  }
}
