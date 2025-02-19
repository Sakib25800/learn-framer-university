"use client"

import { cva, type VariantProps } from "class-variance-authority"
import { twMerge } from "tailwind-merge"
import { HTMLMotionProps, motion } from "framer-motion"

const button = cva(
  ["inline-flex", "justify-center", "items-center", "rounded-full", "font-medium", "cursor-pointer", "!text-white"],
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
  }
)

export interface ButtonProps
  extends Omit<HTMLMotionProps<"button">, keyof VariantProps<typeof button>>,
    VariantProps<typeof button> {
  href?: string
}

export const Button = ({ className, intent, size, href, ...props }: ButtonProps) => {
  return (
    <motion.button
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
    </motion.button>
  )
}
