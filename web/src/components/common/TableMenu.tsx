import {
  Divider,
  ListItemIcon,
  ListItemText,
  Menu,
  MenuItem,
} from "@mui/material";
import { NavSubmenu, useStore } from "../../utils";
import { Delete, DeleteForever, Settings } from "@mui/icons-material";
import { useState } from "react";
import AlertDialog from "../AlertDialog";

type TableMenuProps = {
  open: boolean;
  item?: NavSubmenu;
  anchorEl: HTMLDivElement | null;
  onClose: () => void;
};

const TableMenu = ({ open, anchorEl, item, onClose }: TableMenuProps) => {
  const setOpenTableConfig = useStore(
    (state) => state.setOpenTableConfigDialog
  );
  const setTableConfig = useStore((state) => state.setCurrentTableConfig);
  const tableConfigs = useStore((state) => state.tableConfigs);

  const [showClearDialog, setShowClearDialog] = useState(false);
  const [showDropDialog, setShowDropDialog] = useState(false);

  return (
    <>
      <Menu
        id="basable-table-menu"
        open={open}
        anchorEl={anchorEl}
        onClose={onClose}
      >
        <MenuItem
          onClick={() => {
            onClose();
            setShowClearDialog(true);
          }}
        >
          <ListItemIcon>
            <Delete fontSize="small" />
          </ListItemIcon>
          <ListItemText>Clear</ListItemText>
        </MenuItem>
        <Divider />
        <MenuItem
          onClick={() => {
            onClose();
            setShowDropDialog(true);
          }}
        >
          <ListItemIcon>
            <DeleteForever fontSize="small" />
          </ListItemIcon>
          <ListItemText>Drop</ListItemText>
        </MenuItem>
        <Divider />
        <MenuItem
          onClick={() => {
            onClose();
            const tc = tableConfigs.find((tc) => tc.name === item?.value);
            if (tc) setTableConfig(tc);

            setOpenTableConfig(true);
          }}
        >
          <ListItemIcon>
            <Settings fontSize="small" />
          </ListItemIcon>
          <ListItemText>Settings</ListItemText>
        </MenuItem>
      </Menu>
      <AlertDialog
        open={showClearDialog}
        title="Clear Table"
        content={`WARNING! This action will clear and delete all data from '${item?.label}' table. The action cannot be undone.`}
        actionText="Proceed"
        onHideDialog={() => setShowClearDialog(false)}
        onProceed={() => console.log("clear")}
      />
      <AlertDialog
        open={showDropDialog}
        title="Drop Table"
        content={`WARNING! This action will delete '${item?.label}' table. The action cannot be undone.`}
        actionText="Proceed"
        onHideDialog={() => setShowDropDialog(false)}
        onProceed={() => console.log("drop")}
      />
    </>
  );
};

export default TableMenu;
