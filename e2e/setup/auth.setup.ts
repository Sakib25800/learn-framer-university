import { test as setup } from "@playwright/test"
import { extractContinueToken, readLatestEmail } from "../helpers/email"

const USER_AUTH_FILE = "playwright/.auth/user.json"
const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080"

setup("authenticate user", async ({ page, request }) => {
  // Initiate sign in
  await request.post(`${API_URL}/v1/auth/signin`, {
    data: {
      email: "test@example.com",
    },
  })

  const emailContent = await readLatestEmail()

  if (!emailContent) {
    throw new Error("Expected continue email to found")
  }

  const continueToken = extractContinueToken(emailContent)

  if (!continueToken) {
    throw new Error("Expected continue token to found")
  }

  // Continue sign in
  const continueResponse = await request.get(`${API_URL}/v1/auth/continue/${continueToken}`)
  const tokens = await continueResponse.json()

  if (!tokens.access_token || !tokens.refresh_token) {
    throw new Error("Expected access and refresh tokens in continue response")
  }

  await page.context().addCookies([
    {
      name: "access_token",
      value: tokens.access_token,
      domain: new URL(API_URL).hostname,
      path: "/",
      httpOnly: true,
      secure: false,
      sameSite: "Lax",
    },
    {
      name: "refresh_token",
      value: tokens.refresh_token,
      domain: new URL(API_URL).hostname,
      path: "/",
      httpOnly: true,
      secure: false,
      sameSite: "Lax",
    },
  ])

  await page.context().storageState({ path: USER_AUTH_FILE })
})

setup.skip("authenticate admin", () => {
  // TODO: Implement admin authentication
})
