import { Divider, ListItemIcon, ListItemText, Menu, MenuItem } from "@mui/material";
import { NavSubmenu } from "../../utils";
import { Delete, DeleteForever, Recycling, Settings } from "@mui/icons-material";

type TableMenuProps = {
  open: boolean;
  item?: NavSubmenu;
  anchorEl: HTMLDivElement | null;
  onClose: () => void;
};

const TableMenu = ({ open, anchorEl, onClose }: TableMenuProps) => {
  return (
    <Menu
      id="basable-table-menu"
      open={open}
      anchorEl={anchorEl}
      onClose={onClose}
    >
      <MenuItem>
        <ListItemIcon>
          <Delete fontSize="small" />
        </ListItemIcon>
        <ListItemText>Clear</ListItemText>
      </MenuItem>
      <Divider />
      <MenuItem>
        <ListItemIcon>
          <DeleteForever fontSize="small" />
        </ListItemIcon>
        <ListItemText>Drop</ListItemText>
      </MenuItem>
      <Divider />
      <MenuItem>
        <ListItemIcon>
          <Settings fontSize="small" />
        </ListItemIcon>
        <ListItemText>Settings</ListItemText>
      </MenuItem>
    </Menu>
  );
};

export default TableMenu;
