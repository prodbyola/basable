import { BasableFilter, FilterInput } from "../../utils";
import FilterList from "./FilterList";
import { NoFilter } from "./NoFilter";

type ShowListProps = {
  filters: FilterInput[];
  onCreateFilter: () => void;
  onRequestNewFilter: (filter: FilterInput) => void;
  onRemoveFilter: (index: number) => void;
};

const ShowFilterList = ({
  filters,
  onCreateFilter,
  onRequestNewFilter,
  onRemoveFilter,
}: ShowListProps) => {
  return (
    <div className="showFilterList">
      {filters.length ? (
        <FilterList
          filters={filters}
          onAddNewFilter={onRequestNewFilter}
          onRemoveFilter={onRemoveFilter}
        />
      ) : (
        <NoFilter onCreateFilter={onCreateFilter} />
      )}
    </div>
  );
};

export default ShowFilterList;
