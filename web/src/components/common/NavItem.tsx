import { ExpandLess, ExpandMore } from "@mui/icons-material";
import {
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Collapse,
  List,
  ThemeProvider,
} from "@mui/material";
import FiberManualRecordIcon from "@mui/icons-material/FiberManualRecord";
import * as React from "react";
import theme from "../../theme";

const navItemStyle = {
  mt: "10px",
  color: "#363636",
  fontFamily: '"Exo", sans-serif',
  "&.Mui-selected": {
    border: `1px solid`,
    borderColor: "#4451CA",
    borderRadius: "10px",
    color: "#4451CA",
  },
};

interface NavItemProps {
  label: string;
  icon: React.ReactNode;
  selected?: boolean;
  expandable?: boolean;
  onClick?: React.MouseEventHandler;
  expanded?: boolean;
  subMenu?: {
    items: string[];
    active?: string;
  };
  onSubItemClick?: (item: string) => void;
}

export const NavItem = ({
  label,
  icon,
  selected,
  expanded,
  expandable = false,
  onClick,
  subMenu = {
    items: [],
  },
  onSubItemClick,
}: NavItemProps) => {
  return (
    <ThemeProvider theme={theme}>
      <ListItemButton sx={navItemStyle} onClick={onClick} selected={selected}>
        <ListItemIcon
          sx={{
            minWidth: "32px",
          }}
        >
          {icon}
        </ListItemIcon>
        <ListItemText
          primary={label}
          sx={{
            "& .MuiTypography-body1": {
              fontFamily: '"Exo", sans-serif',
              color: "#363636",
            },
          }}
        />
        {expandable && (expanded ? <ExpandLess /> : <ExpandMore />)}
      </ListItemButton>
      {expandable && (
        <Collapse in={expanded} timeout="auto" unmountOnExit>
          <List component="div">
            {subMenu.items.map((item) => (
              <ListItemButton
                key={item}
                sx={{
                  fontFamily: '"Exo", sans-serif',
                  pl: 4,
                  pt: 0,
                  pb: 0,
                  backgroundColor: subMenu.active === item ? theme.palette.primary.main : '',
                  color: subMenu.active === item ? 'white' : ''
                }}
                onClick={() => onSubItemClick(item)}
              >
                <ListItemIcon
                  sx={{
                    "&.MuiListItemIcon-root": {
                      minWidth: "18px",
                    },
                  }}
                >
                  <FiberManualRecordIcon
                    sx={{
                      "&.MuiSvgIcon-root": {
                        width: "4px",
                        height: "4px",
                        fill: subMenu.active === item ? 'white' : '',
                      },
                    }}
                  />
                </ListItemIcon>
                <ListItemText primary={item} />
              </ListItemButton>
            ))}
          </List>
        </Collapse>
      )}
    </ThemeProvider>
  );
};
