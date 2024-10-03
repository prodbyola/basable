import { useNavigate } from "react-router-dom";
import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import { BASE_URL, BASABLE_COOKIE_NAME } from "../env";
import { getCookie } from "./cookies";
import { SessionCookie } from "./data_types";
import { useCallback } from "react";

type RequestMethod = "get" | "post" | "put" | "patch" | "delete";
type RequestOptions = {
  path: string;
  method: RequestMethod;
  data?: unknown;
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
        "Content-Type": "application/json",
      },
    };

    const cs = getCookie(BASABLE_COOKIE_NAME);
    if (cs) {
      const cookie: SessionCookie = JSON.parse(cs);
      const token = "Bearer " + cookie.token;

      if (cookie.isAuth) config.headers["Authorization"] = token;
      else config.headers["session-id"] = token;

      config.headers["connection-id"] = cookie.connID;
    }

    return new Promise(async (resolve, reject) => {
      try {
        let resp: AxiosResponse | undefined = undefined;
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
            resp = await axios.put(url, data, config);
            break;

          case "delete":
            resp = await axios.delete(url, config);
            break;
        }

        return resolve(resp.data);
      } catch (err) {
        return reject(err);
      }
    });
  }
}

export const useNetworkRequest = () => {
  const navigate = useNavigate()

  const makeRequest = useCallback(async(opts: RequestOptions) => {
    const np = NetworkProvider.getInstance()
    try {
      const resp = await np.request(opts)
      return resp
    } catch(err) {
      if(err.status === 403) {
        navigate('')
      } else {
        return err
      }
    }
  }, [navigate])

  return makeRequest
}
