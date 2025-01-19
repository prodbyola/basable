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
  onHideDialog: () => void;
  onDelete: () => void;
};

const DeleteItemsDialog = ({
  open,
  title,
  content,
  onHideDialog,
  onDelete,
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
          onDelete()
        }} variant="contained" autoFocus>
          Delete
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default DeleteItemsDialog