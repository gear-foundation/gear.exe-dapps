import { TypeRegistry } from "@polkadot/types";
import { HexString } from "@/lib/types";
import { useReadContract, useWriteContract } from "wagmi";
import { catDogIdentifierAbi } from "./catDogIdentifierAbi";
import { CAT_IDENTIFIER_CONTRACT_ADDRESS } from "@/consts";
import { mirrorAbi } from "./mirrorAbi";
import { numberArrayToHex } from "../utils";

type Params = {
  onError: () => void;
};

export const useCatsPredictPredict = ({ onError }: Params) => {
  const { writeContract, reset, data, isPending } = useWriteContract();

  const { data: mirror } = useReadContract({
    abi: catDogIdentifierAbi,
    address: CAT_IDENTIFIER_CONTRACT_ADDRESS,
    functionName: "mirror",
  });

  const mirrorId = mirror as HexString;

  const catsPredict = async (pixels: number[]) => {
    // TODO: use fnCnnCatsDogsPredict when contract fixed
    // debugEncodedData(pixels);
    // const bytes = numberArrayToHex(pixels);
    // writeContract(
    //   {
    //     abi: catDogIdentifierAbi,
    //     address: CAT_IDENTIFIER_CONTRACT_ADDRESS,
    //     functionName: "fnCnnCatsDogsPredict",
    //     args: [pixels, true, 0],
    //   },
    //   { onError }
    // );

    const hex = numberArrayToHex(pixels);

    // TODO: remove it
    const registry = new TypeRegistry();
    const continue_execution = true;
    const payload = registry
      .createType("(String, String, Vec<u8>, bool)", [
        "CnnCatsDogs",
        "Predict",
        hex,
        continue_execution,
      ])
      .toHex();

    writeContract(
      {
        abi: mirrorAbi,
        address: mirrorId,
        functionName: "sendMessage",
        args: [payload, 0],
      },
      { onError }
    );
  };

  return {
    mirrorId,
    catsPredict,
    reset,
    data,
    isPredictPending: isPending,
  };
};
