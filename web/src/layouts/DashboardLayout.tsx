import React, { useEffect, useState } from "react";

import DashboardHeader from "../components/common/DashboardHeader";
import DashboardNav from "../components/common/DashboardNav";
import Box from "@mui/material/Box";
import { Outlet } from "react-router-dom";
import {
  getCookie,
  TableConfig,
  TableSummaryType,
  useLogout,
  useNetworkRequest,
  useStore,
} from "../utils";
import { BASABLE_COOKIE_NAME } from "../env";
import { Alert, Snackbar, SnackbarCloseReason } from "@mui/material";
import { isAxiosError } from "axios";

function DashboardLayout() {
  const logout = useLogout();
  const request = useNetworkRequest();
  const updateTables = useStore((state) => state.updateTables);
  const addTableConfig = useStore((state) => state.addTableConfig);

  const snackBar = useStore((state) => state.snackBar);
  const showAlert = useStore((state) => state.showAlert);
  const hideAlert = useStore((state) => state.hideAlert);

  const [isReady, setIsReady] = useState(false);
  const [showSidebar, onShowSidebar] = useState(false);

  const closeAlert = (
    event?: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason
  ) => {
    if (reason === "clickaway") {
      return;
    }

    hideAlert()
  };

  useEffect(() => {
    if (!isReady) {
      const cookie = getCookie(BASABLE_COOKIE_NAME);
      const loadData = async () => {
        if (!cookie) {
          logout();
        } else {
          try {
            const tables = (await request({
              method: "get",
              path: "tables",
            })) as TableSummaryType[];
            updateTables(tables);

            if (tables.length) {
              for (let i = 0; i < tables.length; i++) {
                const tbl = tables[i];
                const config = (await request({
                  method: "get",
                  path: "tables/configurations/" + tbl.name,
                })) as TableConfig;

                addTableConfig(config);
              }
            }

            setIsReady(true);
          } catch (err: any) {
            let msg = err.message;
            if (isAxiosError(err)) {
              msg = err.response?.data;
            }

            showAlert("error", msg);
            logout();
          }
        }
      };

      loadData();
    }
  });

  if (!isReady) {
    return <div></div>;
  }

  return (
    <Box
      sx={{
        display: "flex ",
        flexWrap: {
          xs: "wrap",
          md: "nowrap",
        },
      }}
    >
      <DashboardHeader onShowSidebar={() => onShowSidebar(!showSidebar)} />
      <DashboardNav showMobileSidebar={showSidebar} />
      <Box className="dashboardMainPage" sx={{ width: "100%" }}>
        <Outlet />
      </Box>
      <Snackbar
        anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
        open={snackBar.showAlert}
        autoHideDuration={5000}
        onClose={closeAlert}
      >
        <Alert
          onClose={closeAlert}
          severity={snackBar.alertColor}
          variant="filled"
          sx={{ width: "100%" }}
        >
          {snackBar.message}
        </Alert>
      </Snackbar>
    </Box>
  );
}

export default DashboardLayout;
