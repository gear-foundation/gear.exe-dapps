import { PropsWithChildren } from "react";
import { cn } from "@/lib/utils";
import styles from "./Layout.module.scss";

type Props = PropsWithChildren & {
  className?: string;
};

export const Layout = ({ children, className }: Props) => {
  return <div className={cn(styles.container, className)}>{children}</div>;
};
