import "../../styles/dashboard-main.scss";

import * as React from "react";
import * as d3 from "d3"
import Box from "@mui/material/Box";
import { CardDetails, DashboardCard } from "../../components/DashboardCard";

const dashboardCards: CardDetails[] = [
  { label: "Total Items", value: "142", action: "Show All" },
  { label: "Database Size", value: "5,242,880 mb", action: "Manage" },
  { label: "Host OS", value: "Windows 11" },
  { label: "Server Version", value: "MySQL 8.0.3", action: "Update" },
];

function DashboardMain() {
  return (
    <Box className="dashboardMainPage" sx={{ width: "100%" }}>
      <div className="dashCardList">
        {dashboardCards.map((card) => (
          <DashboardCard
            label={card.label}
            value={card.value}
            action={card.action}
          />
        ))}
      </div>
    </Box>
  );
}

export default DashboardMain;