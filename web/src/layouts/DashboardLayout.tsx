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

function DashboardLayout() {
  const logout = useLogout();
  const request = useNetworkRequest();
  const updateTables = useStore((state) => state.updateTables);
  const addTableConfig = useStore((state) => state.addTableConfig);

  const snackBar = useStore((state) => state.snackBar);
  const updateSnackBar = useStore((state) => state.showSnackBar);

  const [isReady, setIsReady] = useState(false);
  const [showSidebar, onShowSidebar] = useState(false);

  const closeAlert = (
    event?: React.SyntheticEvent | Event,
    reason?: SnackbarCloseReason
  ) => {
    if (reason === "clickaway") {
      return;
    }

    updateSnackBar({
      ...snackBar,
      showAlert: false,
    });
  };

  useEffect(() => {
    if (!isReady) {
      const cookie = getCookie(BASABLE_COOKIE_NAME);

      if (!cookie) {
        logout();
      } else {
        request({
          method: "get",
          path: "tables",
        }).then((resp) => {
          const tables = resp as TableSummaryType[];
          updateTables(tables);

          if (tables.length) {
            tables.forEach((tbl) => {
              request({
                method: "get",
                path: "tables/configurations/" + tbl.name,
              }).then((config) => addTableConfig(config as TableConfig));
            });
          }
        });

        setIsReady(true);
      }
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
