import { HexString } from "@gear-js/api";
import { TypeRegistry } from "@polkadot/types";
import { useQuery } from "@tanstack/react-query";
import { useReadContract } from "wagmi";
import { Result } from "./lib";
import { CONTRACT_ADDRESS, GEAR_API_NODE } from "../consts";
import { abi } from "../assets/abi";

const RESPONSE_SIZE = 100000;

export const readRpcState = async (
  mirrorId?: HexString,
  startIndex = 0,
  endIndex = RESPONSE_SIZE
) => {
  if (!mirrorId) return [];

  const types: Record<string, any> = {
    FixedPoint: { num: "i64", scale: "u32" },
    Point: { c_re: "FixedPoint", c_im: "FixedPoint" },
    Result: {
      c_re: "FixedPoint",
      c_im: "FixedPoint",
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
    "(String, String, Vec<Result>)",
    json.result.payload
  );

  let data = result[2].toJSON() as unknown as Array<Result>;

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

  return { rpcState: data, rpcStatePending: isPending, refetch };
};
