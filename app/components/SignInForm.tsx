"use client"

import { signIn } from "@/lib/auth/actions"
import { ActionState } from "@/lib/middleware"
import { useActionState } from "react"
import { useSearchParams } from "next/navigation"

export const SignInForm = () => {
  const searchParams = useSearchParams()
  const errorFromQuery = searchParams.get("error")

  const [state, formAction, pending] = useActionState<ActionState, FormData>(signIn, {
    error: errorFromQuery || "",
    success: "",
  })

  return (
    <div className="flex h-screen items-center justify-center">
      <form action={formAction} className="flex flex-col gap-2">
        <input
          type="email"
          placeholder="john.doe@example.com"
          name="email"
          className="border border-red-100 text-white"
        />
        {(state.error || errorFromQuery) && <div className="text-sm text-red-500">{state.error || errorFromQuery}</div>}
        {state.success && <div className="text-sm text-green-500">{state.success}</div>}
        <button disabled={pending} type="submit" className="cursor-pointer bg-white">
          Sign In
        </button>
      </form>
    </div>
  )
}
