import * as React from "react";
import { useParams } from "react-router-dom";
import {
  useNetworkRequest,
  TableColumn,
  TableRow,
  TableConfig,
  useStore,
} from "../../utils";
import { IconButton, ThemeProvider, Typography } from "@mui/material";
import theme from "../../theme";

import ReportIcon from "@mui/icons-material/Report";
import SettingsIcon from "@mui/icons-material/Settings";
import SaveIcon from "@mui/icons-material/Save";
import DownloadIcon from "@mui/icons-material/Download";
import TableRefresh from "../../components/common/icons/RefreshIcon";
import TableFilterIcon from "../../components/common/icons/FilterIcon";
import TableSearchIcon from "../../components/common/icons/SearchIcon";

const DatabaseTable = () => {
  const request = useNetworkRequest();
  const { tableID } = useParams();

  const tableConfigs = useStore((state) => state.tableConfigs);
  const [tableConfig, setTableConfig] = React.useState<Partial<TableConfig>>(
    {}
  );
  const [hasUniqueColumn, setHasUniqueColumn] = React.useState(false);

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

    if (tableID) {
      loadData();

      const tc = tableConfigs.find((c) => c.name === tableID);
      if (tc) {
        setTableConfig(tc);
        setHasUniqueColumn(typeof tc.pk_column === "string");
      }
    }
  }, [request, tableID]);

  if (loading) return <div>Loading</div>;

  return (
    <ThemeProvider theme={theme}>
      <div className="displayTableHeader">
        <div
          className="tableConfig"
          style={{
            backgroundColor: theme.palette.primary.main,
            color: "white",
          }}
        >
          <SettingsIcon />
          <h3>{tableID}</h3>
        </div>
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
      {!hasUniqueColumn && (
        <div className="tableHeaderWarning">
          <ReportIcon />
          <Typography>
            No <strong>unique column</strong> is found for this table. Table
            modification is impossible. You can manually set unique column by
            clicking the settings button above.
          </Typography>
        </div>
      )}
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
