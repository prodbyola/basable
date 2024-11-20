import { Button } from "@mui/material";
import AddIcon from '@mui/icons-material/Add';

type NoFilterProps = {
    onCreateFilter: () => void
}

export const NoFilter = ({ onCreateFilter }: NoFilterProps) => {
  return (
    <div className="noFilter">
      <h3>No Filter</h3>
      <Button variant="contained" size="large" startIcon={ <AddIcon /> } onClick={onCreateFilter}>
        Create Filter
      </Button>
    </div>
  );
};
