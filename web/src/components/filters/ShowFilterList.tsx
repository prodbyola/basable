import { BasableFilter, FilterInput } from "../../utils";
import FilterList from "./FilterList";
import { NoFilter } from "./NoFilter";

type ShowListProps = {
  filters: FilterInput[];
  onCreateFilter: () => void;
  onRequestNewFilter: (filter: FilterInput) => void;
};

const ShowFilterList = ({ filters, onCreateFilter, onRequestNewFilter }: ShowListProps) => {
  return (
    <div className="showFilterList">
      {filters.length ? (
        <FilterList filters={filters} onAddNewFilter={ onRequestNewFilter } />
      ) : (
        <NoFilter onCreateFilter={onCreateFilter} />
      )}
    </div>
  );
};

export default ShowFilterList;
