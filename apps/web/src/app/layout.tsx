import "./globals.css";
import type { Metadata } from "next";
import Providers from "./providers";

export const metadata: Metadata = {
  title: "Learn with Framer University",
  description: "Learn everything there is to know about Framer.",
  icons: [
    {
      rel: "icon",
      type: "image/png",
      url: "/images/icon-light.png",
      media: "(prefers-color-scheme: light)",
    },
    {
      rel: "icon",
      type: "image/png",
      url: "/images/icon-dark.png",
      media: "(prefers-color-scheme: dark)",
    },
  ],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-black antialiased">
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
