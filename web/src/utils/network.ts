import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { BASE_URL, BASABLE_COOKIE_NAME } from "../env";
import { getCookie } from "./cookies";
import { SessionCookie } from "./data_types";

type RequestMethod = "get" | "post" | "put" | "patch" | "delete";
export type RequestOptions = {
  path: string;
  method: RequestMethod;
  data?: unknown;
  headers?: AxiosRequestConfig["headers"];
};

export class NetworkProvider {
  private static instance: NetworkProvider;

  private constructor() {}

  static getInstance(): NetworkProvider {
    if (!NetworkProvider.instance) {
      NetworkProvider.instance = new NetworkProvider();
    }

    return NetworkProvider.instance;
  }

  async request<R>(opts: RequestOptions): Promise<R> {
    const { method, path, data } = opts;
    const url = BASE_URL + path;

    const config: AxiosRequestConfig = {
      headers: {
        ...opts.headers,
        "Content-Type": "application/json",
      },
    };

    const cs = getCookie(BASABLE_COOKIE_NAME);
    if (cs) {
      const cookie: SessionCookie = JSON.parse(cs);
      const token = "Bearer " + cookie.token;

      if (config.headers) {
        if (cookie.isAuth) config.headers["Authorization"] = token;
        else config.headers["session-id"] = token;

        config.headers["connection-id"] = cookie.connID;
      }
    }

    try {
      let resp: AxiosResponse<R>;
      switch (method) {
        case "get":
          resp = await axios.get(url, config);
          break;

        case "post":
          resp = await axios.post(url, data, config);
          break;

        case "put":
          resp = await axios.put(url, data, config);
          break;

        case "patch":
          resp = await axios.patch(url, data, config);
          break;

        case "delete":
          resp = await axios.delete(url, config);
          break;

        default:
          throw new Error(`Unsupported method ${method}`);
      }

      return resp?.data;
    } catch (err) {
      throw err;
    }
  }
}
