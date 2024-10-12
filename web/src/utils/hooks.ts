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
        if ([403, 412].includes(err.status)) {
          deleteCookie(BASABLE_COOKIE_NAME)
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
