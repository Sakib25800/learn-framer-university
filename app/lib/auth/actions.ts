"use server"

import { z } from "zod"
import api from "../api"
import { validatedAction } from "../middleware"
import { cookies } from "next/headers"
import { redirect } from "next/navigation"

const signInSchema = z.object({
  email: z.string().email().min(3).max(255),
})

export const signIn = validatedAction(signInSchema, async ({ email }) => {
  const { data, error } = await api.signIn(email)

  if (error) return { error: error.detail }

  return {
    success: data.message,
  }
})

export const signOut = async () => {
  const cookieStore = await cookies()

  cookieStore.delete("accessToken")
  cookieStore.delete("refreshToken")

  redirect("/sign-in")
}
