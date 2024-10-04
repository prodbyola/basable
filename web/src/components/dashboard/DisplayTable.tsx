import * as React from "react";
import { useStore } from "../../utils";

const getDate = (d: string) => (new Date(d).toLocaleString())

export const DisplayTable = () => {
  const tableHeaders = [
    "SN",
    "Name",
    "Total Rows",
    "Total Columns",
    "Date Created",
    "Last Updated",
  ];

  const tables = useStore((state) => state.tables)

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
              <td>{tb.row_count.toLocaleString()}</td>
              <td>{tb.col_count.toLocaleString()}</td>
              <td>{getDate(tb.created)}</td>
              <td>{getDate(tb.updated)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
};
