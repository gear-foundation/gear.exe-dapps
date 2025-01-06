import * as React from "react";

import { useScrambleTextOnHover } from "@/lib/hooks/use-scramble-text";
import { mergeRefs } from "@/lib/utils";
import { Button, ButtonProps } from "@/components/ui/Button";

type Props = Omit<ButtonProps, "ref"> & {
  targetText: string;
  chars?: string;
  icon?: React.ReactNode;
};

export const EncryptButton = React.forwardRef<HTMLButtonElement, Props>(
  (
    { targetText, chars, icon, state, disabled, isLoading, children, ...props },
    ref
  ) => {
    const { ref: scrambleRef, replay } = useScrambleTextOnHover(targetText);

    const isDisabled =
      disabled || isLoading || ["disabled", "loading"].includes(state || "");

    return (
      <Button
        onMouseEnter={isDisabled ? undefined : replay}
        ref={mergeRefs([ref, scrambleRef])}
        isLoading={isLoading}
        disabled={isDisabled}
        state={state}
        {...props}
      >
        {icon}
        {targetText}
      </Button>
    );
  }
);
EncryptButton.displayName = "EncryptButton";
