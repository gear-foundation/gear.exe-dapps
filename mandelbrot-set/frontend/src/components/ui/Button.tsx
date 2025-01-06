import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";
import LoadingIcon from "@/assets/icons/loading.svg?react";

const buttonVariants = cva(
  [
    "btn",
    /* Disabled */
    "data-[disabled]:opacity-50",
    /* Focus Visible */
    "data-[focus-visible]:outline-none data-[focus-visible]:ring-2 data-[focus-visible]:ring-ring data-[focus-visible]:ring-offset-2",
  ],
  {
    variants: {
      variant: {
        default: "bg-foreground text-background",
        outline: "bg-background text-foreground border border-foreground/15",
        link: "link border-0 rounded-none",
        icon: "bg-background text-foreground border border-foreground/15",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        secondary:
          "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      },
      size: {
        default: "pt-[13px] pb-[15px] px-[23px]",
        sm: "h-9 px-3 rounded-md",
        icon: "",
      },
      state: {
        default: "",
        loading: "",
        disabled: "",
      },
    },
    compoundVariants: [
      {
        variant: "icon",
        className: "p-[15px]",
      },
      {
        variant: "link",
        className: "p-0 justify-start",
      },

      // Active states
      {
        state: "default",
        className: "interactive-focus",
      },
      {
        variant: "default",
        state: "default",
        className: "active:bg-foreground/50",
      },
      {
        variant: "outline",
        state: "default",
        className: "active:text-foreground/50",
      },
      {
        variant: "icon",
        state: "default",
        className:
          "hover:text-primary focus-visible:text-primary active:text-foreground/50",
      },
      {
        variant: "icon",
        state: ["disabled", "loading"],
        className: "text-foreground/30",
      },

      // states loading/disabled
      {
        state: "loading",
        className: "py-[13px]",
      },
      {
        state: ["disabled", "loading"],
        variant: "default",
        className: "bg-foreground/30",
      },
      {
        state: ["disabled", "loading"],
        variant: ["outline", "icon"],
        className: "text-foreground/30",
      },
      {
        state: ["disabled", "loading"],
        className: "cursor-auto",
      },
    ],
    defaultVariants: {
      variant: "default",
      size: "default",
      state: "default",
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
  isLoading?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      children,
      className,
      variant,
      size,
      isLoading,
      disabled,
      type = "button",
      asChild = false,
      state,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "button";
    const isDisabled =
      disabled || isLoading || ["disabled", "loading"].includes(state || "");

    return (
      <Comp
        className={cn(
          buttonVariants({
            variant,
            size,
            state:
              state ??
              (isLoading ? "loading" : disabled ? "disabled" : "default"),
          }),
          className
        )}
        ref={ref}
        disabled={asChild ? undefined : isDisabled}
        aria-disabled={isDisabled ? "true" : undefined}
        type={asChild ? undefined : type}
        children={
          isLoading ? <LoadingIcon className="animate-spin" /> : children
        }
        {...props}
      />
    );
  }
);
Button.displayName = "Button";

export { Button, buttonVariants };
