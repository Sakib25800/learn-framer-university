import Link from "next/link"
import { env } from "../../env.mjs"

const fetchData = async () => {
  try {
    const data = await fetch(`${env.NEXT_API_URL}/api/v1`)
    const json = await data.json()
    return json
  } catch (error) {
    console.error(error)
    return null
  }
}

export default async function AnotherPage() {
  const data = await fetchData()

  return (
    <div className="min-w-screen min-h-screen">
      <Link href="/">Back</Link>
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
    </div>
  )
}
