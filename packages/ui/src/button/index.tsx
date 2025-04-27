"use client";

import { cva } from "class-variance-authority";
import { twMerge } from "tailwind-merge";
import { LazyMotion, domAnimation, m } from "motion/react";
import type { HTMLMotionProps } from "motion/react";

const button = cva(
  [
    "inline-flex",
    "justify-center",
    "items-center",
    "rounded-full",
    "font-medium",
    "cursor-pointer",
    "!text-white",
  ],
  {
    variants: {
      intent: {
        primary: ["bg-transparent", "border", "border-primary-700"],
        secondary: ["bg-accent"],
      },
      size: {
        sm: ["text-base", "px-4", "py-2"],
        md: ["text-body", "px-5", "py-2.5"],
        lg: ["text-body-lg", "px-[26px]", "py-[15px]"],
      },
    },
    defaultVariants: {
      intent: "primary",
      size: "sm",
    },
  },
);

export interface ButtonProps
  extends Omit<HTMLMotionProps<"button">, "formAction"> {
  href?: string;
  intent?: "primary" | "secondary";
  size?: "sm" | "md" | "lg";
  formAction?: string;
}

export const Button = ({
  className,
  intent,
  size,
  href,
  ...props
}: ButtonProps) => {
  return (
    <LazyMotion features={domAnimation}>
      <m.button
        className={twMerge(button({ intent, size, className }))}
        whileHover={{ opacity: 0.8 }}
        whileTap={{ scale: 0.95 }}
        transition={{
          duration: 0.2,
          ease: "easeInOut",
        }}
        onClick={href ? () => (window.location.href = href) : undefined}
        {...props}
      >
        {props.children}
      </m.button>
    </LazyMotion>
  );
};
