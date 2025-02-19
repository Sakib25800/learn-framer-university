import createClient from "openapi-fetch"
import type { paths } from "../shared/api/v1"
import { env } from "../env"

const client = createClient<paths>({ baseUrl: env.NEXT_PUBLIC_API_URL })

export default client
