import { HexString } from "@gear-js/api";
import { TypeRegistry } from "@polkadot/types";
import { useQuery } from "@tanstack/react-query";
import { useReadContract } from "wagmi";
import { PointResult } from "./lib";
import { CONTRACT_ADDRESS, GEAR_API_NODE } from "../consts";
import { abi } from "../assets/abi";

const RESPONSE_SIZE = 100000;

export const readRpcState = async (
  mirrorId?: HexString,
  startIndex = 0,
  endIndex = RESPONSE_SIZE
) => {
  if (!mirrorId) return [];
  console.log("read state from startIndex:", startIndex);

  const types: Record<string, any> = {
    PointResult: {
      c_re: "i128",
      c_im: "i128",
      iter: "u32",
      checked: "bool",
    },
  };

  const registry = new TypeRegistry();
  registry.setKnownTypes({ types });
  registry.register(types);

  const payload = registry
    .createType("(String, String, u32, u32)", [
      "Manager",
      "GetResults",
      startIndex,
      endIndex,
    ])
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
    "(String, String, Vec<PointResult>)",
    json.result.payload
  );

  let data = result[2].toJSON() as unknown as Array<PointResult>;

  if (data?.length === RESPONSE_SIZE) {
    let newData = await readRpcState(
      mirrorId,
      startIndex + RESPONSE_SIZE,
      endIndex + RESPONSE_SIZE
    );
    if (newData) {
      data = [...data, ...newData];
    }
  }

  return data;
};

export const useReadRpcState = () => {
  const { data: mirrorId } = useReadContract({
    abi,
    address: CONTRACT_ADDRESS,
    functionName: "mirror",
  });

  const { data, isPending, refetch } = useQuery({
    queryKey: ["readState", mirrorId],
    queryFn: async () => await readRpcState(mirrorId as HexString),
  });

  const retryWhileDataChanged = () =>
    new Promise<void>((resolve) => {
      const isEmptyPrevData = data?.length === 0;

      const retry = async (atempt = 0) => {
        const response = await refetch();
        const isEmptyNextData = response.data?.length === 0;
        const isDataChanged = isEmptyPrevData !== isEmptyNextData;

        if (isDataChanged) {
          console.log("resolved on atempt", atempt);
          resolve();
        } else {
          setTimeout(() => {
            retry(atempt + 1);
          }, 1000);
        }
      };

      retry();
    });

  return {
    rpcState: data,
    rpcStatePending: isPending,
    refetch,
    retryWhileDataChanged,
  };
};
