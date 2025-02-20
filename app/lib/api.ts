import createClient, { Middleware } from "openapi-fetch"
import type { paths } from "../../shared/api/v1"
import { env } from "../../env"
import { cookies } from "next/headers"
import { redirect } from "next/navigation"

const PUBLIC_ROUTES = ["/v1/auth/signin", "/v1/auth/continue/{token}"]

const authMiddleware: Middleware = {
  async onRequest({ request }) {
    const cookieStore = await cookies()
    const accessToken = cookieStore.get("access_token")?.value
    const isPublicRoute = PUBLIC_ROUTES.includes(request.url)

    if (!accessToken && !isPublicRoute) {
      redirect(`/sign-in?error=${encodeURIComponent("Verification failed")}`)
    }

    request.headers.set("Authorization", `Bearer ${accessToken}`)

    return request
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
