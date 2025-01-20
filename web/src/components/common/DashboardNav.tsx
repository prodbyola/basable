import * as React from "react";
import { ListItemText } from "@material-ui/core";
import ToolBar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import Drawer from "@mui/material/Drawer";
import Divider from "@mui/material/Divider";
import ListItem from "@mui/material/ListItem";
import MenuItem from "@mui/material/MenuItem";
import { Avatar, ListSubheader } from "@mui/material";
import { ThemeProvider } from "@mui/material/styles";
import theme from "../../theme";
import database1 from "../../assets/images/database1.svg";
import database2 from "../../assets/images/database2.svg";
import database3 from "../../assets/images/database3.svg";
import database4 from "../../assets/images/database4.svg";

import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

import { DashboardIcon } from "./icons/DashboardIcon";
import { TablesIcon } from "./icons/TablesIcon";
import { VisualizationIcon } from "./icons/VisualizationIcon";
import { HelpIcon } from "./icons/HelpIcon";
import { LogoutIcon } from "./icons/LogoutIcon";
import { NavItem } from "./NavItem";
import { getTableLabel, NavSubmenu, useLogout, useStore } from "../../utils";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import TableMenu from "./TableMenu";

const drawerWidth = 240;

function DashboardNav({ showMobileSidebar = false }) {
  const currentUser = useStore((state) => state.currentUser);
  const tableConfigs = useStore((state) => state.tableConfigs);
  const location = useLocation();
  const { tableID } = useParams();
  const defaultTextColor = "#363636";

  const navigate = useNavigate();
  const logout = useLogout();

  const isDashboardRoute = location.pathname === "/dashboard";
  const isTableRoute = location.pathname.startsWith("/dashboard/tables/");

  const [openTables, setOpenTables] = React.useState(true);
  const [tableMenuAnchor, setTableMenuAnchor] =
    React.useState<HTMLDivElement | null>(null);
  const [ tableRightClick, setTableRightClick ] = React.useState<NavSubmenu | undefined>(undefined)

  return (
    <ThemeProvider theme={theme}>
      <Drawer
        variant="permanent"
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          display: {
            xs: showMobileSidebar ? "flex" : "none",
            md: "flex",
          },
        }}
      >
        <ToolBar sx={{ height: "100px" }}></ToolBar>
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
            overflow: "auto",
            padding: "20px",
            height: "calc(100% - 100px)",
          }}
        >
          <div className="main-container">
            <ListItem>
              <Avatar
                sx={{
                  bgcolor: theme.palette.primary.main,
                  color: "white",
                  marginRight: "12px",
                }}
              >
                {currentUser.dp ? (
                  <img src={currentUser.dp} alt="avatar" />
                ) : (
                  currentUser.name[0]
                )}
              </Avatar>
              <ListItemText
                primary={
                  <strong
                    style={{
                      fontWeight: "700",
                      fontFamily: '"Exo", sans-serif',
                    }}
                  >
                    {currentUser.name}
                  </strong>
                }
                secondary={
                  <span
                    style={{
                      display: "inline-flex",
                      paddingLeft: "5px",
                      paddingRight: "5px",
                      backgroundColor: "red",
                      borderRadius: "5px",
                      color: "white",
                      fontFamily: '"Exo", sans-serif',
                    }}
                  >
                    {currentUser.role}
                  </span>
                }
              />
            </ListItem>
            <ListItem
              sx={{ paddingLeft: "0px", fontFamily: '"Exo", sans-serif' }}
            >
              <Box>
                <FormControl fullWidth>
                  <Select
                    defaultValue={10}
                    sx={{
                      "& .MuiOutlinedInput-notchedOutline": {
                        border: "none",
                      },
                      "& .MuiInputBase-input": {
                        fontFamily: '"Exo", sans-serif',
                        display: "flex",
                      },
                    }}
                  >
                    <ListSubheader>
                      <strong style={{ fontFamily: '"Exo", sans-serif' }}>
                        Connected Database
                      </strong>
                    </ListSubheader>
                    <Box
                      sx={{ paddingX: "15px", fontFamily: '"Exo", sans-serif' }}
                    >
                      <Divider />
                    </Box>
                    <MenuItem
                      value={10}
                      sx={{ fontFamily: '"Exo", sans-serif' }}
                    >
                      <img
                        src={database1}
                        alt=""
                        style={{ paddingRight: "5px" }}
                      />
                      Basable Database
                    </MenuItem>
                    <MenuItem value={20}>
                      <img
                        src={database2}
                        alt=""
                        style={{ paddingRight: "5px" }}
                      />
                      Jumia Database
                    </MenuItem>
                    <MenuItem value={30}>
                      <img
                        src={database3}
                        alt=""
                        style={{ paddingRight: "5px" }}
                      />
                      Whatsapp Database
                    </MenuItem>
                    <MenuItem value={40}>
                      <img
                        src={database4}
                        alt=""
                        style={{ paddingRight: "5px" }}
                      />
                      Instagram Database
                    </MenuItem>
                  </Select>
                </FormControl>
              </Box>
            </ListItem>
            <Divider />
            <NavItem
              label="Dashboard"
              icon={
                <DashboardIcon
                  color={
                    isDashboardRoute
                      ? theme.palette.primary.main
                      : defaultTextColor
                  }
                />
              }
              selected={isDashboardRoute}
              onClick={() => navigate("/dashboard")}
            />
            <NavItem
              label="Tables"
              icon={
                <TablesIcon
                  color={
                    isTableRoute ? theme.palette.primary.main : defaultTextColor
                  }
                />
              }
              expandable
              expanded={openTables}
              selected={isTableRoute}
              onClick={() => setOpenTables(!openTables)}
              subMenu={{
                items: tableConfigs.map((c) => ({
                  label: getTableLabel(c),
                  value: c.name,
                })),
                active: tableID,
              }}
              onSubItemClick={(item) =>
                navigate("/dashboard/tables/" + item.value)
              }
              onSubItemShowMenu={(el, item) => {
                setTableMenuAnchor(el)
                setTableRightClick(item)
              }}
              key={tableConfigs.length}
            />

            <NavItem label="Visualization" icon={<VisualizationIcon />} />
          </div>
          <div className="bottom-container">
            <NavItem label="LogOut" icon={<LogoutIcon />} onClick={logout} />
            <NavItem label="Help" icon={<HelpIcon />} />
          </div>
          <TableMenu
            open={Boolean(tableMenuAnchor)}
            anchorEl={tableMenuAnchor}
            item={tableRightClick}
            onClose={() => setTableMenuAnchor(null)}
          />
        </Box>
      </Drawer>
    </ThemeProvider>
  );
}

export default DashboardNav;
