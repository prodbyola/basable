import "../../styles/dashboard-main.scss";

import * as React from "react";
import Box from "@mui/material/Box";
import { CardDetails, DashboardCard } from "../../components/DashboardCard";
import { TableGraph } from "../../components/dashboard/TableGraph";
import { DisplayTable } from "../../components/dashboard/DisplayTable";
import { deleteCookie, getCookie, useNetworkRequest } from "../../utils";
import { BASABLE_COOKIE_NAME } from "../../env";
import { useNavigate } from "react-router-dom";
import { useStore } from "../../utils";

const dashboardCards: CardDetails[] = [
  { label: "Total Items", value: "142", action: "Show All" },
  { label: "Database Size", value: "5,242,880 mb", action: "Manage" },
  { label: "Host OS", value: "Windows 11" },
  { label: "Server Version", value: "MySQL 8.0.3", action: "Update" },
];

function DashboardMain() {
  const navigate = useNavigate()
  const request = useNetworkRequest()
  const updateTables = useStore(state => state.updateTables)

  React.useEffect(() => {
    const cookie = getCookie(BASABLE_COOKIE_NAME)
    if(!cookie) {
      deleteCookie(BASABLE_COOKIE_NAME)
      navigate('')
      return
    }

    request({
      method: 'get',
      path: 'table-summaries'
    }).then(tables => (updateTables(tables)))

    request({
      method: 'get',
      path: 'server'
    }).then(resp => console.log(resp))

  }, [request, navigate, updateTables])
 
  return (
    <Box className="dashboardMainPage" sx={{ width: "100%" }}>
      <div className="dashCardList">
        {dashboardCards.map((card) => (
          <DashboardCard
            label={card.label}
            value={card.value}
            action={card.action}
            key={card.label}
          />
        ))}
      </div>
      <TableGraph />
      <DisplayTable />
    </Box>
  );
}

export default DashboardMain;
