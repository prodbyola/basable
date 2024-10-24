import * as React from "react";
import { useParams } from "react-router-dom";
import {
  useNetworkRequest,
  TableColumn,
  TableRow,
  TableConfig,
  useStore,
  getTableLabel,
  UpdateTableData,
} from "../../utils";
import { IconButton, ThemeProvider, Typography } from "@mui/material";
import theme from "../../theme";

import ReportIcon from "@mui/icons-material/Report";
import FilterAltIcon from '@mui/icons-material/FilterAlt';
import SettingsIcon from "@mui/icons-material/Settings";
import SearchIcon from '@mui/icons-material/Search';
import SaveIcon from "@mui/icons-material/Save";
import DownloadIcon from "@mui/icons-material/Download";
import TableRefresh from "../../components/common/icons/RefreshIcon";
import TableSearchIcon from "../../components/common/icons/SearchIcon";
import TableConfigForm from "../../components/forms/TableConfigForm";

const DatabaseTable = () => {
  const request = useNetworkRequest();
  const { tableID } = useParams();

  const tableConfigs = useStore((state) => state.tableConfigs);
  const showAlert = useStore((state) => state.showAlert);

  const [tableConfig, setTableConfig] = React.useState<Partial<TableConfig>>(
    {}
  );
  const [tableLabel, setTableLabel] = React.useState("");
  const [hasUniqueColumn, setHasUniqueColumn] = React.useState(false);
  const [openTableConfig, setOpenTableConfig] = React.useState(false);

  const [columns, setColumns] = React.useState<TableColumn[]>([]);
  const [rows, setRows] = React.useState<TableRow[]>([]);

  const defaultUTD: UpdateTableData = {
    columns: [],
    unique_values: [],
    input: [],
  };
  const [utd, setUTD] = React.useState(defaultUTD);

  const [tableLoading, setTableLoading] = React.useState(false);

  const getColumnValue = (name: string, row: TableRow) => {
    const o = row[name];
    const k = Object.keys(row[name])[0];
    return o[k] as string;
  };

  const getInputLabel = (row: TableRow) => {
    if (hasUniqueColumn) {
      return getColumnValue(tableConfig.pk_column as string, row);
    }

    return "edit-table-input";
  };

  const onInputChange = (
    evt: React.ChangeEvent<HTMLInputElement>,
    column: string,
    rowIndex: number
  ) => {
    const { name: uniqueValue, value } = evt.target;

    // update table rows
    const row = rows[rowIndex];
    row[column][0] = value;
    rows.splice(rowIndex, 1, row);
    setRows([...rows]);

    const uniqueValues = utd.unique_values;
    const exists = uniqueValues.find((uv) => uv === uniqueValue);

    // if column is not added yet, add it
    const columns = utd.columns;
    if (!columns.find((col) => col === column)) {
      columns.push(column);
      utd.columns = columns;
    }

    // if row exists update the row
    if (exists) {
      const i = uniqueValues.indexOf(exists);
      const row = utd.input[i];
      row[column] = value;

      utd.input.splice(i, 1, row);
    } else {
      utd.unique_values.push(uniqueValue);
      utd.input.push({ [column]: value });
    }
    
    setUTD({ ...utd });
  };

  const updateConfigStates = (config: TableConfig) => {
    setTableConfig(config);
    setHasUniqueColumn(typeof config.pk_column === "string");
    setTableLabel(getTableLabel(config as TableConfig));

    if (typeof config.pk_column === "string") {
      setUTD({
        ...utd,
        unique_key: config.pk_column,
      });
    }
  };

  const updateData = async () => {
    if(!utd.unique_values.length) return

    try {
      await request({
        method: "patch",
        path: "tables/data/" + tableID,
        data: utd,
      });

      showAlert("success", "Table data saved successfully");

      setUTD({
        ...defaultUTD,
        unique_key: tableConfig.pk_column
      })
    } catch (err: any) {
      showAlert("error", err.message);
    }
  };

  React.useEffect(() => {
    const loadData = async () => {
      setTableLoading(true);
      const cols = (await request({
        method: "get",
        path: "tables/columns/" + tableID,
      })) as TableColumn[];

      const tc = tableConfigs.find((c) => c.name === tableID);
      if (tc) {
        // if there's a unique column, always shift it to leftmost
        if(tc.pk_column) {
          for(let i = 0; i < cols.length; i++) {
            const col = cols[i]
            if(col.name === tc.pk_column) {
              cols.splice(i, 1)
              cols.splice(0, 0, col)
              break;
            }
          }
        }

        updateConfigStates(tc);
      }
      setColumns(cols);

      const rows = (await request({
        method: "get",
        path: "tables/data/" + tableID,
      })) as TableRow[];
      setRows(rows);

      setTableLoading(false);
    };

    if (tableID) loadData();
  }, [request, tableID]);

  if (tableLoading) return <div>Loading</div>;

  return (
    <ThemeProvider theme={theme}>
      <div className="displayTableHeader">
        <div
          className="tableConfig"
          key={tableLabel}
          style={{
            backgroundColor: theme.palette.primary.main,
            color: "white",
          }}
          onClick={() => setOpenTableConfig(true)}
        >
          <SettingsIcon />
          <h3>{tableLabel}</h3>
        </div>
        <div className="tableToolbar">
          <IconButton>
            <DownloadIcon />
          </IconButton>
          <IconButton>
            <TableRefresh />
          </IconButton>
          <IconButton>
            <SearchIcon />
          </IconButton>
          <IconButton>
            <FilterAltIcon />
          </IconButton>
          <IconButton sx={{
            backgroundColor: utd.unique_values.length ? theme.palette.primary.main : '',
            color: utd.unique_values.length ? 'white' : ''
          }} onClick={updateData}>
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
                        name={getInputLabel(row)}
                        value={getColumnValue(col.name, row)}
                        onChange={(evt) => onInputChange(evt, col.name, index)}
                        disabled={!hasUniqueColumn}
                      />
                    }
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </section>
      <TableConfigForm
        config={tableConfig}
        open={openTableConfig}
        columns={columns.map((col) => col.name)}
        onHideDialog={() => setOpenTableConfig(false)}
        onConfigUpdated={(config) => updateConfigStates(config as TableConfig)}
      />
    </ThemeProvider>
  );
};

export default DatabaseTable;
