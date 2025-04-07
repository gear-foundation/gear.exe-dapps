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

  const pixels = imageData.data; // Array of RGBA values (4 elements per pixel)
  const grayscaleArray = [];

  for (let i = 0; i < pixels.length; i += 4) {
    if (pixels[i] === 0) {
      grayscaleArray.push(0);
      continue;
    }

    const isUseDeviation = Math.random() > 0.5;
    const middleBrightness = 250;
    const deviationRange = 5;
    const deviationSpread = deviationRange * 2;
    const getRandomDeviation = () =>
      Math.round(Math.random() * deviationSpread - deviationRange);
    const noisedBrightness = isUseDeviation
      ? middleBrightness + getRandomDeviation()
      : middleBrightness;

    grayscaleArray.push(Math.min(noisedBrightness, 255));
  }

  return grayscaleArray; // Array of 784 elements
};

export const findMaxIndex = (numbers: number[]): number | null => {
  if (numbers.length === 0) {
    return null;
  }

  return numbers.reduce((maxIndex, currentValue, currentIndex, array) => {
    return currentValue > array[maxIndex] ? currentIndex : maxIndex;
  }, 0);
};
