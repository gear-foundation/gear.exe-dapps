import { useProgram as useGearJsProgram } from "@gear-js/react-hooks";
import { Program } from "./lib";
import { HexString } from "@gear-js/api";

const useProgram = (programId?: HexString) => {
  const { data: program } = useGearJsProgram({
    library: Program,
    id: programId,
  });

  return program;
};

export { useProgram };
