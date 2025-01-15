import { useWatchContractEvent, useWriteContract } from "wagmi";
import { digitRecognitionAbi } from "./DigitRecognitionAbi";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS } from "@/consts";
import { useState } from "react";

type Params = {
  onSuccess: () => void;
};

export const useDigitRecognitionPredict = ({ onSuccess }: Params) => {
  const { writeContract, reset, data, isPending } = useWriteContract();

  const [isSubmiting, setIsSubmiting] = useState(false);

  useWatchContractEvent({
    abi: digitRecognitionAbi,
    eventName: "DigitRecognitionPredictReply",
    address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
    onLogs() {
      onSuccess?.();
      setIsSubmiting(false);
    },
  });

  const digitRecognitionPredict = async (pixels: number[]) => {
    setIsSubmiting(true);
    writeContract(
      {
        abi: digitRecognitionAbi,
        address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
        functionName: "fnDigitRecognitionPredict",
        args: [pixels, 0],
      },
      { onError: () => setIsSubmiting(false) }
    );
  };

  return {
    digitRecognitionPredict,
    reset,
    data,
    isPredictPending: isPending || isSubmiting,
  };
};
