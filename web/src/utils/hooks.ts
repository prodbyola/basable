import { useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { deleteCookie, NetworkProvider, RequestOptions, useStore } from ".";
import { BASABLE_COOKIE_NAME } from "../env";
import axios from "axios";

export const useNetworkRequest = <R>() => {
  const navigate = useNavigate();

  const makeRequest = useCallback(
    async (opts: RequestOptions) => {
      const np = NetworkProvider.getInstance();
      try {
        const resp: R = await np.request(opts);
        return resp;
      } catch (err: any) {
        if (axios.isAxiosError(err)) {
          if (err.status && [403, 412].includes(err.status)) {
            deleteCookie(BASABLE_COOKIE_NAME);
            navigate("");
          }
          
          throw new Error(err.response?.data)
        } else {
          throw err;
        }
      }
    },
    [navigate]
  );

  return makeRequest;
};

export const useLogout = () => {
  const navigate = useNavigate();
  const storeLogout = useStore((state) => state.logout);

  const logout = () => {
    deleteCookie(BASABLE_COOKIE_NAME);
    storeLogout();
    navigate("/");
  };

  return logout;
};
