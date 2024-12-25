import { Button, Card } from "@/components";
import { DigitCanvas } from "./DigitCanvas";
import { useRef, useState } from "react";
import { getFlattenedPixelArray } from "./utils";
import { useDigitRecognitionPredict } from "./api/useDigitRecognitionPredict";
import styles from "./DigitRecognizer.module.scss";

export const DigitRecognizer = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isCanvasTouched, setIsCanvasTouched] = useState(false);
  const { digitRecognitionPredict, reset, data, isPending } =
    useDigitRecognitionPredict();

  const clearCanvas = () => {
    const canvas = canvasRef.current;
    if (canvas) {
      const ctx = canvas.getContext("2d");
      if (ctx) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        setIsCanvasTouched(false);
      }
    }
  };

  const onReset = () => {
    clearCanvas();
    reset();
  };

  const predictedDigit = data === undefined ? null : Number(data);

  const onSubmit = () => {
    const flattenedPixelArray = getFlattenedPixelArray(canvasRef);
    // digitRecognitionPredict(flattenedPixelArray);
    console.log("🚀 ~ onSubmit ~ flattenedPixelArray:", flattenedPixelArray);
  };

  return (
    <Card
      title="Digit recognizer"
      description={
        predictedDigit === null
          ? "Use the dotted canvas to draw any number from 0 to 9. Submit your drawing and let the AI recognize your handwritten digit instantly."
          : `Most confident answer is ${predictedDigit}.`
      }
      canvasSlot={
        <DigitCanvas
          canvasRef={canvasRef}
          isTouched={isCanvasTouched}
          onTouchedChange={setIsCanvasTouched}
        />
      }
      headerSlot={
        <Button
          variant="outline"
          size="xs"
          className={styles.headerButton}
          onClick={clearCanvas}
        >
          Clear
        </Button>
      }
      footer={
        <>
          {predictedDigit === null && isCanvasTouched && (
            <Button
              className={styles.footerButton}
              onClick={onSubmit}
              isLoading={isPending}
            >
              Submit
            </Button>
          )}
          {predictedDigit !== null && (
            <Button
              variant="outline"
              className={styles.footerButton}
              onClick={onReset}
              isLoading={isPending}
            >
              Start again
            </Button>
          )}
        </>
      }
    />
  );
};
