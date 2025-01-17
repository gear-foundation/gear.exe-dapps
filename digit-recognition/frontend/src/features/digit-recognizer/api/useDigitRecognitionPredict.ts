import { useWatchContractEvent, useWriteContract } from "wagmi";
import { digitRecognitionAbi } from "./DigitRecognitionAbi";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS } from "@/consts";

type Params = {
  onSuccess: () => void;
  onError: () => void;
};

export const useDigitRecognitionPredict = ({ onSuccess, onError }: Params) => {
  const { writeContract, reset, data, isPending } = useWriteContract();

  useWatchContractEvent({
    abi: digitRecognitionAbi,
    eventName: "DigitRecognitionPredictReply",
    address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
    onLogs() {
      onSuccess();
    },
  });

  const digitRecognitionPredict = async (pixels: number[]) => {
    writeContract(
      {
        abi: digitRecognitionAbi,
        address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
        functionName: "fnDigitRecognitionPredict",
        args: [pixels, 0],
      },
      { onError }
    );
  };

  return {
    digitRecognitionPredict,
    reset,
    data,
    isPredictPending: isPending,
  };
};
