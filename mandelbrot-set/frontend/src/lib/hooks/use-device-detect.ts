import { useEffect } from "react";
import { atom, useAtomValue, useSetAtom } from "jotai";

import { isMobileDevice } from "@/lib/utils";

const mobileAtom = atom(false);

export function useSyncIsMobileAtom() {
  const setIsMobile = useSetAtom(mobileAtom);

  useEffect(() => {
    setIsMobile(isMobileDevice());
  }, []);
}

export function useIsMobileAtom() {
  return useAtomValue(mobileAtom);
}
