import {
  BaseError,
  useWriteContract,
  useWatchContractEvent,
  useAccount,
} from "wagmi";
import { useState } from "react";
import { abi } from "@/assets/abi";
import { CONTRACT_ADDRESS } from "@/consts";
import { useReadRpcState } from "@/api/readRpcState";

import { Button, Canvas, Input, Layout, StatePreview } from "@/components";
import { HashLink } from "../ui/HashLink";

export const ComputationForm = () => {
  const ethAccount = useAccount();
  const [isLoading, setIsLoading] = useState(false);
  const [maxIter, setMaxIter] = useState(0);
  const [width, setWidth] = useState(0);
  const [height, setHeight] = useState(0);

  const { writeContract, isPending } = useWriteContract();

  const { refetch, rpcState, rpcStatePending } = useReadRpcState();

  useWatchContractEvent({
    abi,
    address: CONTRACT_ADDRESS,
    eventName: "ManagerGenerateAndStorePointsReply",
    batch: false,
    onLogs(logs) {
      console.log("ManagerGenerateAndStorePointsReply", logs);
      setIsLoading(false);
      refetch();
    },
  });

  useWatchContractEvent({
    abi,
    address: CONTRACT_ADDRESS,
    eventName: "ManagerRestartReply",
    batch: false,
    onLogs(logs) {
      console.log("ManagerRestartReply", logs);
      setIsLoading(false);
      refetch();
    },
  });

  const onSuccess = () => {
    console.log("SUCCESS");
  };

  const onError = (error: Error) => {
    const errorMessage = (error as BaseError).shortMessage || error.message;

    console.error(error);
    alert(errorMessage);
    setIsLoading(false);
  };

  const onRestart = () => {
    setIsLoading(true);
    writeContract(
      {
        abi,
        address: CONTRACT_ADDRESS,
        functionName: "fnManagerRestart",
        args: [0],
      },
      {
        onSuccess,
        onError,
      }
    );
  };

  const onGenerateAndStorePoints = () => {
    setIsLoading(true);
    writeContract(
      {
        abi,
        address: CONTRACT_ADDRESS,
        functionName: "fnManagerGenerateAndStorePoints",
        args: [
          width,
          height,
          { num: -2, scale: 0 },
          { num: 1, scale: 0 },
          { num: -15, scale: 1 },
          { num: 15, scale: 1 },
          30000,
          true,
          true,
          1000,
          20,
          0,
        ],
      },
      {
        onSuccess,
        onError,
      }
    );
  };

  const chainName = ethAccount.chain!.name;
  const hasState = rpcState && rpcState.length > 0;
  const pending = rpcStatePending || isPending || isLoading;

  return (
    <Layout className="flex flex-col gap-5">
      <h1>Distributed Computation</h1>

      <HashLink
        hash={CONTRACT_ADDRESS}
        href={`https://${chainName}.etherscan.io/address/${CONTRACT_ADDRESS}`}
      />

      <Input
        name="maxIter"
        value={maxIter}
        onChange={(event) => {
          setMaxIter(Number(event.currentTarget.value) || 0);
        }}
        disabled={hasState}
      />

      <Input
        name="width"
        value={width}
        onChange={(event) => {
          setWidth(Number(event.currentTarget.value) || 0);
        }}
        disabled={hasState}
      />

      <Input
        name="height"
        value={height}
        onChange={(event) => {
          setHeight(Number(event.currentTarget.value) || 0);
        }}
        disabled={hasState}
      />

      <div style={{ display: "flex", gap: "12px" }}>
        {hasState ? (
          <Button
            variant="outline"
            onClick={onRestart}
            isLoading={pending}
            className="w-52"
          >
            Cleanup Contract state
          </Button>
        ) : (
          <Button
            variant="outline"
            onClick={onGenerateAndStorePoints}
            isLoading={pending}
            className="w-50"
          >
            Start calculation
          </Button>
        )}
      </div>

      {hasState && (
        <>
          <p>Contract state (length: {rpcState.length})</p>
          <Canvas nodes={rpcState} />
          <StatePreview nodes={rpcState} />
        </>
      )}
    </Layout>
  );
};
