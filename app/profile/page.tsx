import { SignOutButton } from "@/components/SignOutButton"
import api from "@/lib/api"

export default async function Profile() {
  const { data, error } = await api.getUser()

  if (error) {
    return <div className="text-red-500">Error: {error.title}</div>
  }

  return (
    <div>
      <h1>Profile</h1>
      <pre className="text-white">{JSON.stringify(data, null, 2)}</pre>
      <SignOutButton />
    </div>
  )
}
