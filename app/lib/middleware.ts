/* eslint-disable @typescript-eslint/no-explicit-any */

import { z } from "zod"

export type ActionState = {
  error?: string
  success?: string
  // Allows for additional properties
  [key: string]: any
}

type ValidatedActionFunction<S extends z.ZodType<any, any>, T> = (data: z.infer<S>, formData: FormData) => Promise<T>

export const validatedAction = <S extends z.ZodType<any, any>, T>(schema: S, action: ValidatedActionFunction<S, T>) => {
  return async (_prevState: ActionState, formData: FormData): Promise<T> => {
    const result = schema.safeParse(Object.fromEntries(formData))
    if (!result.success) {
      return {
        error: result.error.errors
          .map((err) => {
            return `${err.path.join(".")}: ${err.message}`
          })
          .join(", "),
      } as T
    }

    return action(result.data, formData)
  }
}
