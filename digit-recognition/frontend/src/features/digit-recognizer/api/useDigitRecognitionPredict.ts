import { useWriteContract } from "wagmi";
import { digitRecognitionAbi } from "./DigitRecognitionAbi";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS } from "@/consts";

type Params = {
  onError: () => void;
};

export const useDigitRecognitionPredict = ({ onError }: Params) => {
  const { writeContract, data, isPending } = useWriteContract();

  const digitRecognitionPredict = async (pixels: number[]) => {
    writeContract(
      {
        abi: digitRecognitionAbi,
        address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
        functionName: "fnDigitRecognitionPredict",
        args: [pixels, true, 0],
      },
      { onError }
    );
  };

  return {
    digitRecognitionPredict,
    data,
    isPredictPending: isPending,
  };
};
