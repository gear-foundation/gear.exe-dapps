import { FixedPoint } from "./types";

const expectedSize = 28;

export const getFlattenedPixelArray = (
  canvasRef: React.RefObject<HTMLCanvasElement>
): number[] => {
  if (!canvasRef.current) return [];

  const originalCanvas = canvasRef.current;
  const originalCtx = originalCanvas.getContext("2d");
  if (!originalCtx) return [];

  const tempCanvas = document.createElement("canvas");
  tempCanvas.width = expectedSize;
  tempCanvas.height = expectedSize;
  const tempCtx = tempCanvas.getContext("2d");
  if (!tempCtx) return [];

  tempCtx.drawImage(originalCanvas, 0, 0, expectedSize, expectedSize);

  const imageData = tempCtx.getImageData(0, 0, expectedSize, expectedSize);

  const pixels = imageData.data; // Array of RGBA values ​​(4 elements per pixel)
  const grayscaleArray = [];

  for (let i = 0; i < pixels.length; i += 4) {
    const r = pixels[i];
    const g = pixels[i + 1];
    const b = pixels[i + 2];
    const brightness = Math.round((r + g + b) / 3);

    grayscaleArray.push(brightness > 0 ? 1 : 0);
  }

  return grayscaleArray; // Array of 784 elements
};

export const getFloatingPoint = ({ num, scale }: FixedPoint) => {
  return Number(Number(num) / Math.pow(10, Number(scale)));
};

export const findMaxIndex = (numbers: number[]): number | null => {
  if (numbers.length === 0) {
    return null;
  }

  return numbers.reduce((maxIndex, currentValue, currentIndex, array) => {
    return currentValue > array[maxIndex] ? currentIndex : maxIndex;
  }, 0);
};
