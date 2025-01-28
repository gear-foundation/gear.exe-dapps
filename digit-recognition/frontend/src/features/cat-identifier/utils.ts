const expectedSize = 128;

export const handleImageToPixels = async (imageSrc: string) => {
  const img = new Image();
  const tempCanvas = document.createElement("canvas");
  const ctx = tempCanvas.getContext("2d");

  return new Promise<number[]>((resolve, reject) => {
    img.onload = () => {
      tempCanvas.width = expectedSize;
      tempCanvas.height = expectedSize;

      if (!ctx) return reject();

      ctx.drawImage(img, 0, 0, expectedSize, expectedSize);

      const imageData = ctx.getImageData(0, 0, expectedSize, expectedSize);
      const rgbaPixels = imageData.data;

      const rgbPixels = [];

      for (let i = 0; i < rgbaPixels.length; i += 4) {
        const r = rgbaPixels[i];
        const g = rgbaPixels[i + 1];
        const b = rgbaPixels[i + 2];

        rgbPixels.push(r, g, b);
      }

      resolve(rgbPixels);
    };

    img.onerror = reject;
    img.src = imageSrc;
  });
};

export const numberArrayToHex = (array: number[]) => {
  return "0x" + array.map((b) => b.toString(16).padStart(2, "0")).join("");
};
