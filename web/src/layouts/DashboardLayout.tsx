import React, { useState } from "react";

import DashboardHeader from "../components/common/DashboardHeader";
import DashboardNav from "../components/common/DashboardNav";
import Box from "@mui/material/Box";
import { Outlet } from "react-router-dom";

function DashboardLayout() {
  const [showSidebar, onShowSidebar] = useState(false);
  
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
      <Outlet />
    </Box>
  );
}

export default DashboardLayout;
