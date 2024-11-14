import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import { BasableFilter } from "../../utils";

type FilterListProps = {
  filters: BasableFilter[];
};

const FilterList = ({ filters }: FilterListProps) => {
  const tableCols = ["S/N", "Combinator", "Column", "Operator", "Value", "Actions"];
  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            {tableCols.map((col) => (
              <TableCell key={col} align="right">
                {col}
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {filters.map((filter, index) => (
            <TableRow
              key={index}
              sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
            >
                <TableCell>{ index + 1 }</TableCell>
                <TableCell>{ filter.filterType.toUpperCase() }</TableCell>
                <TableCell>{ filter.column }</TableCell>
                <TableCell>{ filter.operatorKey }</TableCell>
                <TableCell>{ filter.filterValue }</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default FilterList;