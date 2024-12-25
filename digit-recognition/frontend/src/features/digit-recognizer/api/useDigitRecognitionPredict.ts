import { useWriteContract } from "wagmi";
import { digitRecognitionAbi } from "./DigitRecognitionAbi";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS } from "@/consts";

export const useDigitRecognitionPredict = () => {
  const { writeContract, reset, data, isPending } = useWriteContract();

  const digitRecognitionPredict = async (pixels: number[]) => {
    writeContract({
      abi: digitRecognitionAbi,
      address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
      functionName: "fnDigitRecognitionPredict",
      args: [pixels, true, 0],
    });
  };

  return { digitRecognitionPredict, reset, data, isPending };
};
