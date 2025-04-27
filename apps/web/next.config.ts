import { NextConfig } from "next";
import { initOpenNextCloudflareForDev } from "@opennextjs/cloudflare";
import withBundleAnalyzer from '@next/bundle-analyzer';

const nextConfig: NextConfig = {
  typescript: {
    ignoreBuildErrors: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  outputFileTracingExcludes: {
    "*": [
      "./**/*.js.map",
      "./**/*.mjs.map",
      "./**/*.cjs.map"
    ],
  },
};

export default withBundleAnalyzer({
  enabled: process.env.ANALYZE === 'true',
})(nextConfig);

initOpenNextCloudflareForDev();
