import api from "@/lib/api"
import { NextResponse } from "next/server"
import { cookies } from "next/headers"

export async function GET(request: Request, { params }: { params: { token: string } }) {
  const { token } = params

  const { data, error } = await api.continueSignIn(token)

  if (error) {
    return NextResponse.redirect(new URL("/sign-in?error=verification-failed", request.url))
  }

  const cookieOptions = {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    path: "/",
    sameSite: "lax" as const,
    maxAge: 60 * 60 * 24 * 7, // 7 days
  }

  const response = NextResponse.redirect(new URL("/profile", request.url))

  const cookieStore = await cookies()

  cookieStore.set("accessToken", data.access_token, cookieOptions)
  cookieStore.set("refreshToken", data.refresh_token, cookieOptions)

  return response
}
