import type { NextConfig } from "next"
import { env } from "./env.mjs"

const nextConfig: NextConfig = {
  async rewrites() {
    return [
      {
        source: "/api/:path*",
        destination: `${env.BACKEND_URL}/api/:path*`,
      },
    ]
  },
}

export default nextConfig
