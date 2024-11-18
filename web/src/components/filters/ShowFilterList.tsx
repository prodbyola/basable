import { BasableFilter, FilterInput } from "../../utils";
import FilterList from "./FilterList";
import { NoFilter } from "./NoFilter";

type ShowListProps = {
  filters: FilterInput[];
  onCreateFilter: () => void;
};

const ShowFilterList = ({ filters, onCreateFilter }: ShowListProps) => {
  return (
    <div className="showFilterList">
      {filters.length ? (
        <FilterList filters={filters} />
      ) : (
        <NoFilter onCreateFilter={onCreateFilter} />
      )}
    </div>
  );
};

export default ShowFilterList;
