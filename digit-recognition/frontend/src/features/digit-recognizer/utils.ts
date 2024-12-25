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

  // Debug function to show the preview of the resized image
  //   const showPreview = () => {
  //     const parent = originalCanvas.parentElement;
  //     canvasRef.current?.remove();
  //     parent?.append(tempCanvas);
  //   };
  //   showPreview();

  const imageData = tempCtx.getImageData(0, 0, expectedSize, expectedSize);

  const pixels = imageData.data;
  const grayscaleArray = [];

  // Array of RGBA values ​​(4 elements per pixel)
  for (let i = 0; i < pixels.length; i += 4) {
    const r = pixels[i];
    const g = pixels[i + 1];
    const b = pixels[i + 2];
    const brightness = Math.round((r + g + b) / 3);

    grayscaleArray.push(brightness > 0 ? 1 : 0);
  }

  return grayscaleArray; // Array of 784 elements
};
