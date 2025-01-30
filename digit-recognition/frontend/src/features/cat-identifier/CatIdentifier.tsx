import { Button, Card } from "@/components";
import { useRef, useState } from "react";
import { handleImageToPixels } from "./utils";
import { useReadRpcState } from "./api/readRpcState";
import { getFloatingPoint } from "@/lib/utils";
import { useCatsPredictPredict } from "./api/useCatsPredictPredict";
import { PROBABILITY_EDGE } from "./consts";
import styles from "./CatIdentifier.module.scss";

export const CatIdentifier = () => {
  const [image, setImage] = useState<string | null>(null);
  const [isSubmited, setIsSubmited] = useState(false);
  const [isSubmiting, setIsSubmiting] = useState(false);
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const onSuccess = () => {
    setIsSubmiting(false);
    setIsSubmited(true);
  };

  const onError = () => setIsSubmiting(false);

  const { rpcState, rpcStatePending } = useReadRpcState({
    isSubmiting,
    onSuccess,
  });
  const { catsPredict, reset, mirrorId } = useCatsPredictPredict({ onError });

  const probability =
    isSubmited && rpcState && rpcState.calculated
      ? getFloatingPoint(rpcState.probability)
      : null;

  const isPending = rpcStatePending || isSubmiting;

  const result = (() => {
    if (probability === null) {
      return null;
    }

    return (
      <div>
        Cat in the image:{" "}
        <span className={styles.result}>
          {probability < PROBABILITY_EDGE ? "recognized" : "not recognized  "}
        </span>{" "}
        (confidence score {((1 - probability) * 100).toFixed(2)}%).
      </div>
    );
  })();

  const onFileChange = (file: File | undefined) => {
    if (file) {
      const reader = new FileReader();
      reader.onload = () => {
        const imgSrc = reader.result as string;
        setImage(imgSrc);
      };
      reader.readAsDataURL(file);
    }
  };

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    onFileChange(file);
  };

  const handleDragOver = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
  };

  const handleDrop = (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const file = event.dataTransfer.files[0];
    onFileChange(file);
  };

  const onSubmit = async () => {
    if (!image) return;
    setIsSubmiting(true);
    const pixels = await handleImageToPixels(image);
    catsPredict(pixels);
  };

  const onReset = () => {
    setIsSubmited(false);
    reset();
  };

  return (
    <Card
      title="Cat identifier"
      description={
        result ??
        "Upload any image to see if the AI detects a cat. The model will analyze the picture and let you know if a cat is recognized."
      }
      address={mirrorId}
      canvasSlot={
        <div
          onDragOver={handleDragOver}
          onDrop={handleDrop}
          className={styles.container}
        >
          {image ? (
            <img
              src={image}
              alt="Uploaded"
              style={{
                maxWidth: "100%",
                maxHeight: "100%",
              }}
            />
          ) : (
            <p>OR DRAG AND DROP IMAGE HERE</p>
          )}
        </div>
      }
      headerSlot={
        isSubmited || isPending ? null : (
          <>
            <Button
              onClick={() => fileInputRef.current?.click()}
              variant="outline"
              size="xs"
              className={styles.headerButton}
            >
              {image ? "Change file" : "Select file"}
            </Button>
            <input
              type="file"
              ref={fileInputRef}
              style={{ display: "none" }}
              accept="image/*"
              onChange={handleFileChange}
            />
          </>
        )
      }
      footer={
        <>
          {image && result === null && (
            <Button
              className={styles.footerButton}
              onClick={onSubmit}
              isLoading={isPending}
            >
              Submit
            </Button>
          )}
          {result !== null && (
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
