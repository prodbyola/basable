import { Button, ButtonGroup } from "@mui/material";
import { FilterInput, sampleFilter } from "../../utils";

type FilterListProps = {
  filters: FilterInput[];
  onAddNewFilter: (filter: FilterInput) => void;
};

const FilterList = ({ filters, onAddNewFilter }: FilterListProps) => {
  return (
    <>
      <div className="filterList">
        {filters.map((filter, index) => (
          <div className="filterItem">
            <p>{index + 1}</p>
            <p>{filter.column}</p>
            <p className="filterOperator">{filter.operatorLabel}</p>
            <p>{filter.operatorValue}</p>
            <p>{filter.combinator.toUpperCase()}</p>
            <div className="deleteIcon">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                height="24px"
                viewBox="0 0 24 24"
                width="24px"
                fill="#5f6368"
              >
                <path d="M0 0h24v24H0V0z" fill="none" />
                <path d="M16 9v10H8V9h8m-1.5-6h-5l-1 1H5v2h14V4h-3.5l-1-1zM18 7H6v12c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7z" />
              </svg>
            </div>
          </div>
        ))}
      </div>
      <div className="addMoreFilter">
        <ButtonGroup sx={{ float: "inline-end" }}>
          <Button
            onClick={() =>
              onAddNewFilter({ ...sampleFilter, combinator: "and" })
            }
          >
            AND
          </Button>
          <Button
            onClick={() =>
              onAddNewFilter({ ...sampleFilter, combinator: "or" })
            }
          >
            OR
          </Button>
        </ButtonGroup>
      </div>
    </>
  );
};

export default FilterList;
