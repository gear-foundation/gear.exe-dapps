import { PropsWithChildren } from "react";
import { cn } from "@/lib/utils";
import { Bricks, leftRows, rightRows } from "@/components";
import { useIsXl } from "@/lib/hooks/use-media";

type Props = PropsWithChildren & {
  className?: string;
};

export const Layout = ({ children, className }: Props) => {
  const isXL = useIsXl();
  return (
    <div className="relative flex mx-auto w-screen max-w-screen-2xl">
      {isXL && (
        <Bricks initRows={leftRows} className="sticky left-0 top-[78px]" />
      )}
      <div
        className={cn(
          "w-full max-w-[972px] py-[60px] px-8 lg:px-[200px] mx-auto",
          className
        )}
      >
        {children}
      </div>
      {isXL && (
        <Bricks
          initRows={rightRows}
          className="sticky right-0 top-[78px]"
          timeShift={2500}
        />
      )}
    </div>
  );
};
