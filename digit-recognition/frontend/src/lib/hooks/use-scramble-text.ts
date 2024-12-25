import { useScramble } from "use-scramble"

import { useIsMobileAtom } from "@/lib/hooks/use-device-detect"

export function useScrambleTextOnHover(text: string) {
  return useScramble({
    chance: 1,
    overdrive: false,
    overflow: true,
    playOnMount: false,
    scramble: 10,
    seed: 0,
    speed: 0.7,
    step: 2,
    text,
    tick: 2,
  })
}

export function useScrambleTextOnMount(text: string) {
  const isMobile = useIsMobileAtom()

  const desktopControls = useScramble({
    chance: 0.9,
    overdrive: false,
    overflow: true,
    playOnMount: false,
    range: [65, 125],
    scramble: 53,
    seed: 10,
    speed: 0.8,
    step: 10,
    text,
    tick: 6,
  })

  const mobileControls = useScramble({
    chance: 1,
    overdrive: false,
    overflow: true,
    playOnMount: false,
    range: [65, 125],
    scramble: 23,
    seed: 10,
    speed: 1,
    step: 10,
    text,
    tick: 4,
  })

  return isMobile ? mobileControls : desktopControls
}
