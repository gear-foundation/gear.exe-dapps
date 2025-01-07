import * as React from "react";

import { cn } from "@/lib/utils";

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, label, ...props }, ref) => {
    return (
      <div className="flex flex-col gap-1">
        <label
          className={cn("text-xs", props.disabled && "text-[#FFFFFF80]")}
          htmlFor={props.name}
        >
          {label || props.name}
        </label>
        <input
          type={type}
          id={props.name}
          className={cn(
            "flex w-full border border-muted-foreground bg-background px-4 py-3 text-xs",
            "disabled:border-input disabled:bg-[#FFFFFF0F] disabled:text-input",
            "aria-invalid:border-destructive aria-invalid:placeholder:text-destructive",
            className
          )}
          ref={ref}
          {...props}
        />
      </div>
    );
  }
);
Input.displayName = "Input";

export { Input };
