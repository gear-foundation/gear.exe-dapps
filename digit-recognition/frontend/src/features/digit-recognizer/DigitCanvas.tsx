import React, { useEffect, useState } from "react";
import styles from "./DigitCanvas.module.scss";

type DigitCanvasProps = {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  isTouched: boolean;
  onTouchedChange: (isTouched: boolean) => void;
  disabled?: boolean;
};

export const DigitCanvas = ({
  canvasRef,
  isTouched,
  onTouchedChange,
  disabled,
}: DigitCanvasProps) => {
  const [isDrawing, setIsDrawing] = useState(false);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      const ctx = canvas.getContext("2d");
      if (ctx) {
        const dpr = window.devicePixelRatio || 1;
        const { offsetWidth, offsetHeight } = canvas;

        canvas.width = offsetWidth * dpr;
        canvas.height = offsetHeight * dpr;
        ctx.scale(dpr, dpr);

        ctx.lineWidth = 10;
        const gradient = ctx.createLinearGradient(
          0,
          0,
          offsetWidth,
          offsetHeight
        );
        gradient.addColorStop(0, "#A8F593");
        gradient.addColorStop(1, "#628F55");
        ctx.strokeStyle = gradient;
        ctx.lineCap = "round";
        ctx.lineJoin = "round";
        ctx.imageSmoothingEnabled = true;
      }
    }
  }, []);

  const startDrawing = (e: React.MouseEvent) => {
    const canvas = canvasRef.current;
    if (!canvas || disabled) return;

    const ctx = canvas.getContext("2d");
    if (ctx) {
      ctx.beginPath();
      ctx.moveTo(e.nativeEvent.offsetX, e.nativeEvent.offsetY);
      setIsDrawing(true);
      onTouchedChange(true);
    }
  };

  const draw = (e: React.MouseEvent) => {
    if (!isDrawing) return;
    const canvas = canvasRef.current;
    if (canvas) {
      const ctx = canvas.getContext("2d");
      if (ctx) {
        ctx.lineTo(e.nativeEvent.offsetX, e.nativeEvent.offsetY);
        ctx.stroke();
      }
    }
  };

  const stopDrawing = () => {
    setIsDrawing(false);
  };

  return (
    <div className={styles.wrapper}>
      <div className={styles.drawArea}>
        <canvas
          ref={canvasRef}
          onMouseDown={startDrawing}
          onMouseMove={draw}
          onMouseUp={stopDrawing}
          onMouseLeave={stopDrawing}
          style={{ width: "100%", height: "100%", borderRadius: "50%" }}
        />
        {!isTouched && <div className={styles.drawHint}>Draw a digit here</div>}
      </div>
    </div>
  );
};
