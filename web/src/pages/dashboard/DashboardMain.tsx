import "../../styles/dashboard-main.scss";

import * as React from "react";
import { CardDetails, DashboardCard } from "../../components/DashboardCard";
import { TableGraph } from "../../components/dashboard/TableGraph";
import { DisplayTable } from "../../components/dashboard/DisplayTable";
import { ServerDetails, TableSummaryType, useNetworkRequest } from "../../utils";
import { useNavigate } from "react-router-dom";
import { useStore } from "../../utils";

const dashboardCards: CardDetails[] = [
  { label: "Total Items", value: 0, action: "Show All" },
  { label: "Database Size", value: "", action: "Manage" },
  { label: "Host OS", value: "" },
  { label: "Server Version", value: "", action: "Update" },
];

function DashboardMain() {
  const navigate = useNavigate()
  const request = useNetworkRequest()

  const updateTables = useStore(state => state.updateTables)
  const [ serverDetails, updateServerDetails ] = React.useState(dashboardCards)

  React.useEffect(() => {
    try {
      request({
        method: 'get',
        path: 'table-summaries'
      }).then((tables: TableSummaryType[]) => {
        const totalRows = tables.reduce((accm, item) => (accm + item.row_count), 0)
        
        const updatedDetails = [...serverDetails]
        updatedDetails[0].value = totalRows.toLocaleString()
  
        updateTables(tables)
      })
  
      const getServerDetails = async () => {
        const resp: ServerDetails = await request({
          method: 'get',
          path: 'server'
        })
  
        const updatedDetails = [...serverDetails]
        updatedDetails[1].value = resp.db_size.toLocaleString() + 'MB'
        updatedDetails[2].value = resp.os + ' ' + resp.comment
        updatedDetails[3].value = resp.version
  
        updateServerDetails(updatedDetails)
      }
  
      getServerDetails()
    } catch(err) {
      console.log(err)
    }

  }, [request, navigate, updateTables, serverDetails])
 
  return (
    <>
      <div className="dashCardList">
        {serverDetails.map((card) => (
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
    </>
  );
}

export default DashboardMain;
