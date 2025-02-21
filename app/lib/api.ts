import createClient, { Middleware } from "openapi-fetch"
import type { paths } from "../../shared/api/v1"
import { env } from "../../env"
import { cookies } from "next/headers"
import { redirect } from "next/navigation"

const PUBLIC_ROUTES = ["/v1/auth/signin", "/v1/auth/continue"]

const isPublicRoute = (pathname: string) => PUBLIC_ROUTES.some((route) => pathname.startsWith(route))

const authMiddleware: Middleware = {
  async onRequest({ request }) {
    const cookieStore = await cookies()
    const accessToken = cookieStore.get("access_token")?.value

    // Attempting to make request to authorized route, so check credentials
    if (!accessToken && !isPublicRoute(new URL(request.url).pathname)) {
      redirect(`/sign-in?error=${encodeURIComponent("Verification failed")}`)
    }

    request.headers.set("Authorization", `Bearer ${accessToken}`)

    return request
  },
  async onResponse({ response }) {
    if (response.status === 401) {
      const cookieStore = await cookies()

      cookieStore.delete("access_token")
      cookieStore.delete("refresh_token")

      redirect(`/sign-in?error=${encodeURIComponent("Verification failed")}`)
    }
  },
}

const client = createClient<paths>({
  baseUrl: env.NEXT_PUBLIC_API_URL,
})

client.use(authMiddleware)

const api = {
  getUser: () => client.GET("/v1/users/me"),
  signIn: (email: string) => client.POST("/v1/auth/signin", { body: { email } }),
  continueSignIn: (token: string) => client.GET(`/v1/auth/continue/{token}`, { params: { path: { token } } }),
}

export default api
