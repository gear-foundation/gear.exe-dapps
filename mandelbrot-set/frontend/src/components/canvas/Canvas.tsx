import { useEffect, useMemo, useRef } from "react";
import { FixedPoint, Result } from "../../api/lib";

type Props = {
  nodes: Result[];
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
      const ratio = 1 - iter / MAX_ITER;
      return `rgb(0, ${Math.floor(255 * ratio)}, ${Math.round(128 * ratio)})`;
    };

    const getFloatingPoint = ({ num, scale }: FixedPoint) => {
      return Number(Number(num) / Math.pow(10, Number(scale)));
    };

    nodes.forEach(({ c_re, c_im, iter }) => {
      const x = mapToPixel(getFloatingPoint(c_re), reMin, reMax, width);
      const y = mapToPixel(getFloatingPoint(c_im), imMin, imMax, height);
      ctx.fillStyle = getColor(iter);
      ctx.fillRect(x, y, 1, 1);
    });
  }, []);

  return (
    <canvas ref={ref} width={size} height={size} className="w-full"></canvas>
  );
};
