import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";
import type { paths } from "./types";

const API_URL =
  process.env.NODE_ENV === "development"
    ? "http://localhost:8080"
    : "https://api.frameruniversity.com";

const fetchClient = createFetchClient<paths>({
  baseUrl: API_URL,
});

export const $api = createClient(fetchClient);

export type { paths } from "./types";
