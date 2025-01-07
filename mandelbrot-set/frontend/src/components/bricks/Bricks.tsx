import { useEffect, useRef } from "react";
import { drawAnimatedLine, drawRows, generatePaths } from "./utils";
import { ANIMATION_SPEED, ROW_HEIGHT } from "./consts";

type Props = {
  initRows: number[][];
  timeShift?: number;
  className?: string;
};

export const Bricks = ({ initRows, timeShift = 0, className }: Props) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const width = (canvas.width = canvas.offsetWidth);
    const height = (canvas.height = canvas.offsetHeight);

    // get only rows, that fit in height
    const rows = initRows.slice(0, Math.floor(height / ROW_HEIGHT));
    let paths = generatePaths(rows, width);

    let animationFrameId: number;
    let progress = 0; // Progress of the animation

    let isLineAnimationStarted = false;
    setTimeout(() => {
      isLineAnimationStarted = true;
    }, timeShift);

    const drawAnimation = () => {
      ctx.clearRect(0, 0, width, height);

      drawRows(ctx, rows, width, height);

      if (isLineAnimationStarted) {
        paths.forEach(({ startPoint, sections }, index) => {
          drawAnimatedLine(ctx, startPoint, sections, progress - index * 130);
        });

        progress += ANIMATION_SPEED;

        if (progress > 1000) {
          // Restart animation
          progress = 0;
          paths = generatePaths(rows, width);
        }
      }

      animationFrameId = requestAnimationFrame(drawAnimation);
    };

    drawAnimation();

    return () => cancelAnimationFrame(animationFrameId);
  }, []);

  return (
    <canvas
      ref={canvasRef}
      style={{ width: "234px", height: "calc(100vh - 78px)" }}
      className={className}
    />
  );
};
