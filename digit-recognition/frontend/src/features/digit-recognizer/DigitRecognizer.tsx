import { Button, Card } from "@/components";
import { DigitCanvas } from "./DigitCanvas";
import { useRef, useState } from "react";
import { findMaxIndex, getFlattenedPixelArray } from "./utils";
import { useDigitRecognitionPredict } from "./api/useDigitRecognitionPredict";
import { useReadRpcState } from "./api/readRpcState";
import styles from "./DigitRecognizer.module.scss";
import { getFloatingPoint } from "@/lib/utils";

export const DigitRecognizer = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isCanvasTouched, setIsCanvasTouched] = useState(false);
  const [isSubmited, setIsSubmited] = useState(false);
  const [isSubmiting, setIsSubmiting] = useState(false);

  const { rpcState, rpcStatePending, retryWhileDataChanged } =
    useReadRpcState();

  const onSuccess = () =>
    retryWhileDataChanged().then(() => {
      setIsSubmiting(false);
      setIsSubmited(true);
    });

  const onError = () => setIsSubmiting(false);

  const { digitRecognitionPredict, reset, isPredictPending } =
    useDigitRecognitionPredict({ onSuccess, onError });

  const isPending = rpcStatePending || isPredictPending || isSubmiting;

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
    setIsSubmited(false);
    reset();
  };

  const predictedDigit =
    !isSubmited || rpcState === undefined
      ? null
      : findMaxIndex(rpcState.map(getFloatingPoint));

  const onSubmit = () => {
    setIsSubmiting(true);
    const flattenedPixelArray = getFlattenedPixelArray(canvasRef);
    digitRecognitionPredict(flattenedPixelArray);
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
          disabled={isPending || predictedDigit !== null}
        />
      }
      headerSlot={
        predictedDigit === null &&
        !isPending && (
          <Button
            variant="outline"
            size="xs"
            className={styles.headerButton}
            onClick={clearCanvas}
          >
            Clear
          </Button>
        )
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
