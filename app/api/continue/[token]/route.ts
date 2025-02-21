import api from "@/lib/api"
import { NextResponse } from "next/server"
import { cookies } from "next/headers"
import { redirect } from "next/navigation"
import { env } from "env"

export async function GET(request: Request, { params }: { params: Promise<{ token: string }> }) {
  const { token } = await params

  const { data, error } = await api.continueSignIn(token)

  if (error) {
    return redirect("/sign-in?error=verification-failed")
  }

  const cookieOptions = {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    path: "/",
    sameSite: "lax" as const,
    maxAge: 60 * 60 * 24 * 7, // 7 days
  }

  const response = NextResponse.redirect(new URL("/profile", env.NEXT_PUBLIC_APP_URL))

  const cookieStore = await cookies()

  cookieStore.set("access_token", data.access_token, cookieOptions)
  cookieStore.set("refresh_token", data.refresh_token, cookieOptions)

  return response
}
