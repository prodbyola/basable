import { Button, ButtonGroup } from "@mui/material";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import { ArrowForward } from "@mui/icons-material";

type NavigatorProps = {
  count: number;
  totalPages: number;
  currentPage: number;
  onNavigate: (to: "prev" | "next") => void;
};

const TableNavigator = ({
  count,
  currentPage,
  totalPages,
  onNavigate,
}: NavigatorProps) => {
  return (
    <div className="tableNavigation">
      <h3 className="queryCount">
        Total {count} items found {`(${currentPage + 1} of ${totalPages})`}
      </h3>
      <ButtonGroup
        variant="outlined"
        aria-label="Basic button group"
        size="small"
      >
        <Button onClick={() => onNavigate("prev")} disabled={currentPage === 0 || (totalPages === 0)}>
          <ArrowBackIcon />
        </Button>
        <Button
          onClick={() => onNavigate("next")}
          disabled={currentPage + 1 === totalPages || (totalPages === 0)}
        >
          <ArrowForward />
        </Button>
      </ButtonGroup>
    </div>
  );
};

export default TableNavigator;
