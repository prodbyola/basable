import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from "@mui/material";

type DeleteItemsProps = {
  open: boolean;
  title: string;
  content: string;
  actionText: string
  onHideDialog: () => void;
  onProceed: () => void;
};

const AlertDialog = ({
  open,
  title,
  content,
  actionText,
  onHideDialog,
  onProceed,
}: DeleteItemsProps) => {
  return (
    <Dialog open={open}>
      <DialogTitle>{title}</DialogTitle>
      <DialogContent>
        <DialogContentText>
          {content}
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button onClick={onHideDialog}>Close</Button>
        <Button onClick={() => {
          onHideDialog()
          onProceed()
        }} variant="contained" autoFocus>
          { actionText }
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default AlertDialog