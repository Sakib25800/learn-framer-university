"use client"

import { signOut } from "@/lib/auth/actions"

export const SignOutButton = () => {
  return (
    <button onClick={() => signOut()} className="bg-white">
      Sign out
    </button>
  )
}
