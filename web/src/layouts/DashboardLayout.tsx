import React, { useEffect, useState } from "react";

import DashboardHeader from "../components/common/DashboardHeader";
import DashboardNav from "../components/common/DashboardNav";
import Box from "@mui/material/Box";
import { Outlet } from "react-router-dom";
import { getCookie, useLogout } from "../utils";
import { BASABLE_COOKIE_NAME } from "../env";

function DashboardLayout() {
  const logout = useLogout()

  const [isReady, setIsReady] = useState(false)
  const [showSidebar, onShowSidebar] = useState(false);

  useEffect(() => {
    const cookie = getCookie(BASABLE_COOKIE_NAME)
    
    if(!cookie) logout()
    else setIsReady(true)

  }, [logout])

  if(!isReady) {
    return <div></div>
  }
  
  return (
    <Box
      sx={{
        display: "flex ",
        flexWrap: {
          xs: "wrap",
          md: 'nowrap'
        },
      }}
    >
      <DashboardHeader onShowSidebar={() => onShowSidebar(!showSidebar)} />
      <DashboardNav showMobileSidebar={showSidebar} />
      <Box className="dashboardMainPage" sx={{ width: "100%" }}>
        <Outlet />
      </Box>
    </Box>
  );
}

export default DashboardLayout;
