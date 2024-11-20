import "../../styles/dashboard-main.scss";

import * as React from "react";
import { CardDetails, DashboardCard } from "../../components/DashboardCard";
import { TableGraph } from "../../components/dashboard/TableGraph";
import { DisplayTable } from "../../components/dashboard/DisplayTable";
import { ServerDetails, useNetworkRequest } from "../../utils";
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

  const tables = useStore(state => state.tables)
  const [ serverDetails, updateServerDetails ] = React.useState(dashboardCards)
  const [serverLoaded, setServerLoaded] = React.useState(false)

  React.useEffect(() => {
    try {
      const updatedDetails = [...serverDetails]

      // Update table row count
      if(tables.length && !updatedDetails[0].value) {
        const totalRows = tables.reduce((accm, item) => (accm + item.row_count), 0)
        updatedDetails[0].value = totalRows.toLocaleString()
        updateServerDetails(updatedDetails)
      }
      
      // update server details
      const getServerDetails = async () => {
        const resp: ServerDetails = await request({
          method: 'get',
          path: 'server'
        }) as ServerDetails
  
        updatedDetails[1].value = resp.db_size.toLocaleString() + 'MB'
        updatedDetails[2].value = resp.os + ' ' + resp.comment
        updatedDetails[3].value = resp.version
  
        updateServerDetails(updatedDetails)
        setServerLoaded(true)
      }
  
      if(!serverLoaded) getServerDetails()
    } catch(err) {
      console.log(err)
    }

  }, [request, navigate, serverDetails, tables, serverLoaded])
 
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
