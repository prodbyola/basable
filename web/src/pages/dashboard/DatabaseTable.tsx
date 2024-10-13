import * as React from "react";
import { useParams } from "react-router-dom";
import { useNetworkRequest, TableColumn, TableRow } from "../../utils";
import { IconButton, ThemeProvider } from "@mui/material";
import theme from "../../theme";

import SettingsIcon from "@mui/icons-material/Settings";
import SaveIcon from "@mui/icons-material/Save";
import DownloadIcon from "@mui/icons-material/Download";
import TableRefresh from "../../components/common/icons/RefreshIcon";
import TableFilterIcon from "../../components/common/icons/FilterIcon";
import TableSearchIcon from "../../components/common/icons/SearchIcon";

const DatabaseTable = () => {
  const request = useNetworkRequest();
  const { tableID } = useParams();

  const [columns, setColumns] = React.useState<TableColumn[]>([]);
  const [rows, setRows] = React.useState<TableRow[]>([]);
  const [loading, setLoading] = React.useState(false);

  const getColumnValue = (name: string, row: TableRow) => {
    const o = row[name];
    const k = Object.keys(row[name])[0];
    return o[k] as string;
  };

  React.useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      const cols: TableColumn[] = await request({
        method: "get",
        path: "tables/columns/" + tableID,
      });
      setColumns(cols);

      const rows: TableRow[] = await request({
        method: "get",
        path: "tables/data/" + tableID,
      });
      setRows(rows);
      setLoading(false);
    };

    if (tableID) loadData();
  }, [request, tableID]);

  if (loading) return <div>Loading</div>;

  return (
    <ThemeProvider theme={theme}>
      <div className="displayTableHeader">
      <div className="tableToolbar">
        <IconButton>
          <SettingsIcon />
        </IconButton>
      </div>
        <h3 className="tableName">{tableID}</h3>
        <div className="tableToolbar">
          <IconButton>
            <DownloadIcon />
          </IconButton>
          <IconButton>
            <TableRefresh />
          </IconButton>
          <IconButton>
            <TableSearchIcon />
          </IconButton>
          <IconButton>
            <TableFilterIcon size="18" />
          </IconButton>
          <IconButton>
            <SaveIcon />
          </IconButton>
        </div>
      </div>
      <section className="displayTable dashboardDisplay databaseTable">
        <table>
          <thead>
            <tr>
              {columns.map((col) => (
                <th key={col.name}>{col.name}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, index) => (
              <tr className="editableRow" key={index}>
                {columns.map((col) => (
                  <td key={col.name}>
                    {
                      <input
                        value={getColumnValue(col.name, row)}
                        onChange={() => {}}
                      />
                    }
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </section>
    </ThemeProvider>
  );
};

export default DatabaseTable;
