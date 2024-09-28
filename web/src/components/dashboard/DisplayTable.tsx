import * as React from "react";
import { TableSummaryType } from "../../data_desc";

type DisplayTableProps = {
  tables: TableSummaryType[];
};

const getDate = (d: string) => (new Date(d).toLocaleString())

export const DisplayTable: React.FC<DisplayTableProps> = ({ tables }) => {
  const tableHeaders = [
    "SN",
    "Name",
    "Total Rows",
    "Total Columns",
    "Date Created",
    "Last Updated",
  ];
  return (
    <section className="displayTable dashboardDisplay">
      <h3 className="sectionHeader">Table List</h3>
      <table>
        <thead>
          <tr>
            {tableHeaders.map((hd) => (
              <th key={hd}>{hd}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {tables.map((tb, index) => (
            <tr key={tb.name}>
              <td>{index + 1}</td>
              <td>{tb.name}</td>
              <td>{tb.row_count}</td>
              <td>{tb.col_count}</td>
              <td>{getDate(tb.created)}</td>
              <td>{getDate(tb.updated)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
};
