import { createEnv } from "@t3-oss/env-nextjs"
import { z } from "zod"

export const env = createEnv({
  server: {},
  client: {
    NEXT_API_URL: z.string().min(1),
  },
  runtimeEnv: {
    NEXT_API_URL: process.env.NEXT_API_URL,
  },
})
