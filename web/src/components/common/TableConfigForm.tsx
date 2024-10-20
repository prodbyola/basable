import { Button, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle } from "@mui/material";
import { TableConfig } from "../../utils";

type ConfigProps = {
  config: Partial<TableConfig>;
  open: boolean;
  onHideDialog: () => void
};

const TableConfigForm = ({ config, open, onHideDialog }: ConfigProps) => {
  return (
    <Dialog open={open}>
      <DialogTitle>{config.label} Configuration</DialogTitle>
      <DialogContent>
        <DialogContentText id="alert-dialog-slide-description">
          Let Google help apps determine location. This means sending anonymous
          location data to Google, even when no apps are running.
        </DialogContentText>
      </DialogContent>
      <DialogActions>
          <Button onClick={onHideDialog}>Cancel</Button>
          <Button variant="contained">Update</Button>
        </DialogActions>
    </Dialog>
  );
};

export default TableConfigForm
