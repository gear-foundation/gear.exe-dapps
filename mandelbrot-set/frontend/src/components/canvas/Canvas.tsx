import { useEffect, useMemo, useRef } from "react";
import { PointResult } from "../../api/lib";

type Props = {
  nodes: PointResult[];
};

const MAX_ITER = 100;

export const Canvas = ({ nodes }: Props) => {
  const ref = useRef<HTMLCanvasElement>(null);

  const size = useMemo(() => Math.sqrt(nodes.length), [nodes]);

  useEffect(() => {
    const canvas = ref.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;

    const reMin = -2.0,
      reMax = 1.0;
    const imMin = -1.5,
      imMax = 1.5;

    const mapToPixel = (
      value: number,
      min: number,
      max: number,
      size: number
    ) => {
      return Math.round(((value - min) / (max - min)) * size);
    };

    const getColor = (iter: number) => {
      const ratio = Math.max(1 - iter / MAX_ITER, 0);
      return `rgb(0, ${Math.floor(255 * ratio)}, ${Math.round(128 * ratio)})`;
    };

    const getFloatingPoint = (num: number) => {
      return num / Math.pow(2, 32);
    };

    nodes.forEach(({ c_re, c_im, iter }) => {
      const x = mapToPixel(getFloatingPoint(Number(c_re)), reMin, reMax, width);
      const y = mapToPixel(
        getFloatingPoint(Number(c_im)),
        imMin,
        imMax,
        height
      );
      ctx.fillStyle = getColor(iter);
      ctx.fillRect(x, y, 1, 1);
    });
  }, []);

  return (
    <canvas ref={ref} width={size} height={size} className="w-full"></canvas>
  );
};
