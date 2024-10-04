import { useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { deleteCookie, NetworkProvider, RequestOptions, useStore } from ".";
import { BASABLE_COOKIE_NAME } from "../env";

export const useNetworkRequest = () => {
  const navigate = useNavigate();

  const makeRequest = useCallback(
    async (opts: RequestOptions) => {
      const np = NetworkProvider.getInstance();
      try {
        const resp = await np.request(opts);
        return resp;
      } catch (err) {
        if (err.status === 403) {
          navigate("");
        } else {
          return err;
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
