import { HexString } from "@gear-js/api";
import { TypeRegistry } from "@polkadot/types";
import { useQuery } from "@tanstack/react-query";
import { useReadContract, useWatchContractEvent } from "wagmi";

import { digitRecognitionAbi } from "./DigitRecognitionAbi";
import { DIGIT_RECOGNITION_CONTRACT_ADDRESS, GEAR_API_NODE } from "@/consts";
import { Result } from "../types";
import { mirrorAbi } from "./mirrorAbi";

export const readRpcState = async (mirrorId?: HexString) => {
  if (!mirrorId) return;

  const types: Record<string, any> = {
    FixedPoint: { num: "i64", scale: "u32" },
  };

  const registry = new TypeRegistry();
  registry.setKnownTypes({ types });
  registry.register(types);

  const payload = registry
    .createType("(String, String)", ["DigitRecognition", "Result"])
    .toHex();

  const params = {
    jsonrpc: "2.0",
    id: 1,
    method: "program_calculateReplyForHandle",
    params: {
      source: "0xf823ba3F10922DCca6970D1e012D8701f462Aa33",
      program_id: mirrorId,
      payload: payload,
      value: 0,
    },
  };

  const myHeaders = new Headers();
  myHeaders.append("Content-Type", "application/json");

  const response = await fetch(GEAR_API_NODE, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(params),
  });

  const json = await response.json();

  const result = registry.createType(
    "(String, String, Vec<FixedPoint>)",
    json.result.payload
  );

  let data = result[2].toJSON() as unknown as Result;

  return data;
};

type Params = {
  isSubmiting: boolean;
  onSuccess: () => void;
};

export const useReadRpcState = ({ isSubmiting, onSuccess }: Params) => {
  const { data: mirrorId } = useReadContract({
    abi: digitRecognitionAbi,
    address: DIGIT_RECOGNITION_CONTRACT_ADDRESS,
    functionName: "mirror",
  });

  const { data, isPending, refetch } = useQuery({
    queryKey: ["readState", mirrorId],
    queryFn: async () => await readRpcState(mirrorId as HexString),
    enabled: !!mirrorId,
  });

  useWatchContractEvent({
    abi: mirrorAbi,
    eventName: "StateChanged",
    address: mirrorId as HexString,
    onLogs() {
      if (isSubmiting) {
        console.log("success reply");
        setTimeout(() => {
          onSuccess();
          refetch();
        }, 1000);
      }
    },
    enabled: !!mirrorId,
  });

  return {
    rpcState: data,
    rpcStatePending: isPending,
    refetch,
  };
};
