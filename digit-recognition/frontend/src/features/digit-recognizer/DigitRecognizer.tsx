import { Button, Card } from "@/components";
import { DigitCanvas } from "./DigitCanvas";
import { useEffect, useRef, useState } from "react";
import { findMaxIndex, getFlattenedPixelArray } from "./utils";
import { useDigitRecognitionPredict } from "./api/useDigitRecognitionPredict";
import { useReadRpcState } from "./api/readRpcState";
import styles from "./DigitRecognizer.module.scss";
import { getFloatingPoint } from "@/lib/utils";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS } from "@/consts";

export const DigitRecognizer = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isCanvasTouched, setIsCanvasTouched] = useState(false);
  const [isSubmited, setIsSubmited] = useState(false);
  const [isSubmiting, setIsSubmiting] = useState(false);

  const onSuccess = () => {
    setIsSubmiting(false);
    setIsSubmited(true);
  };
  const onError = () => setIsSubmiting(false);

  const { rpcState, rpcStatePending } = useReadRpcState({
    isSubmiting,
    onSuccess,
  });

  const { digitRecognitionPredict, isPredictPending } =
    useDigitRecognitionPredict({ onError });

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
  };

  const currentState =
    rpcState === undefined
      ? null
      : findMaxIndex(rpcState.map(getFloatingPoint));

  const predictedDigit = isSubmited ? currentState : null;

  useEffect(() => {
    console.log(
      "current state:",
      currentState,
      rpcState?.map(getFloatingPoint),
      rpcState
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const onSubmit = () => {
    setIsSubmiting(true);
    const flattenedPixelArray = getFlattenedPixelArray(canvasRef);
    console.log("flattenedPixelArray:", flattenedPixelArray.join(", "));
    digitRecognitionPredict(flattenedPixelArray);
  };

  return (
    <Card
      title="Digit recognizer"
      address={DIGIT_RECOGNITION_CONTRACT_ADDRESS}
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
        !isPending &&
        isCanvasTouched && (
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
