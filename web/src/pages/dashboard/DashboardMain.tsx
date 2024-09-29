import "../../styles/dashboard-main.scss";

import * as React from "react";
import Box from "@mui/material/Box";
import { CardDetails, DashboardCard } from "../../components/DashboardCard";
import { GraphDataType, TableSummaryType } from "../../utils/data_types";
import { TableGraph } from "../../components/dashboard/TableGraph";
import { DisplayTable } from "../../components/dashboard/DisplayTable";

const dashboardCards: CardDetails[] = [
  { label: "Total Items", value: "142", action: "Show All" },
  { label: "Database Size", value: "5,242,880 mb", action: "Manage" },
  { label: "Host OS", value: "Windows 11" },
  { label: "Server Version", value: "MySQL 8.0.3", action: "Update" },
];

const tables: TableSummaryType[] = [
  {
    name: "data_dictionary",
    row_count: 65,
    col_count: 3,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "encounters",
    row_count: 186,
    col_count: 14,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "organizations",
    row_count: 0,
    col_count: 8,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "patients",
    row_count: 974,
    col_count: 20,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "payers",
    row_count: 10,
    col_count: 7,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
  {
    name: "procedures",
    row_count: 65,
    col_count: 9,
    created: "2024-09-22",
    updated: "2024-09-22",
  },
];

function DashboardMain() {
  const graphData: GraphDataType[] = tables.map((t) => ({ label: t.name, value: t.row_count }))
 
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
      <TableGraph data={graphData} />
      <DisplayTable tables={tables} />
    </Box>
  );
}

export default DashboardMain;
