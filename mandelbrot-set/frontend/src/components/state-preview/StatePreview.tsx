import { PointResult } from "@/api/lib";
import { useMemo } from "react";

type Props = {
  nodes: PointResult[];
};

export const StatePreview = ({ nodes }: Props) => {
  const text = useMemo(
    () => JSON.stringify(nodes).replace(/},{/g, "},\n{"),
    [nodes]
  );

  return (
    <p className="max-h-[728px] overflow-y-auto overflow-x-hidden scrollbar whitespace-pre-wrap text-xs py-3 px-4 bg-[#FFFFFF0F]">
      {text}
    </p>
  );
};
