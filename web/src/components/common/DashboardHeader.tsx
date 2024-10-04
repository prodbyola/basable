import * as React from "react";
import { CssBaseline, List } from "@material-ui/core";
import ListItemText from "@mui/material/ListItemText";
import Button from "@mui/material/Button";
import ToolBar from "@mui/material/Toolbar";
import AppBar from "@mui/material/AppBar";
import ListItem from "@mui/material/ListItem";
import AddCircleIcon from "@mui/icons-material/AddCircle";
import Typography from "@mui/material/Typography";
import { ThemeProvider } from "@mui/material/styles";
import theme from "../../theme";
import Logo from "../../assets/images/Basale-logo-white.svg";
import SearchBar from "../bar/SearchBar";
import useStyles from "../../styles/styles.js";
import { Avatar, Box, IconButton } from "@mui/material";
import { AppNotification } from "../bar/Notification";
import MenuIcon from "@mui/icons-material/Menu";
import { useStore } from "../../utils";

const headerHeight = 80;

function DashboardHeader({onShowSidebar}: { onShowSidebar: React.MouseEventHandler }) {
  const classes = useStyles();
  const currentUser = useStore(state => state.currentUser)

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <AppBar
        position="fixed"
        color="inherit"
        elevation={0}
        variant="outlined"
        sx={{
          zIndex: (theme) => theme.zIndex.drawer + 1,
          height: headerHeight,
        }}
      >
        <ToolBar
          className={classes.toolbar}
          sx={{
            paddingRight: {
              xs: "0px",
            },
            justifyContent: 'normal'
          }}
        >
          <IconButton sx={{ display: { xs: "flex", md: "none" } }} aria-label="menu" onClick={onShowSidebar}>
            <MenuIcon />
          </IconButton>
          <img
            src={Logo}
            className="dashboardLogo"
            alt="Logo"
            style={{ height: 30, marginRight: 'auto' }}
          />
          <SearchBar />
          <div className={classes.headerright}>
            <Button
              variant="outlined"
              sx={{
                padding: "8px",
                borderRadius: "10px",
                marginRight: "50px",
                display: { sm: "flex", xs: "none" },
                width: "auto",
              }}
            >
              <AddCircleIcon color="primary" />
              <Typography
                variant="subtitle2"
                sx={{
                  textTransform: "none",
                  textWrap: "nowrap",
                  marginLeft: "10px",
                  display: { md: "block", sm: "none" },
                  fontWeight: "600",
                }}
              >
                Connect new database
              </Typography>
            </Button>
            <Box
              sx={{
                display: "flex",
                alignItems: "center",
              }}
            >
              <AppNotification />
              <List>
                <ListItem>
                  <Avatar
                    sx={{
                      minWidth: "42px",
                      bgcolor: theme.palette.primary.main,
                      color: 'white',
                      cursor: 'pointer'
                    }}
                  >
                    { 
                      currentUser.dp ? <img
                      src={currentUser.dp}
                      alt="avatar"
                      style={{ width: 30, height: 30 }}
                    /> : currentUser.name[0]
                    }
                    
                  </Avatar>

                  <ListItemText
                    primary="Stefania Asuqo"
                    secondary="stefaniaas@gmail.com"
                    sx={{
                      display: { md: "block" },

                      "& .MuiTypography-root": {
                        fontFamily: '"Exo", sans-serif',
                        display: {
                          xs: "none",
                        },
                      },

                      "& .MuiTypography-body1": {
                        fontWeight: "700",
                        display: {
                          xs: "none",
                        },
                      },
                    }}
                  />
                </ListItem>
              </List>
            </Box>
          </div>
        </ToolBar>
      </AppBar>
    </ThemeProvider>
  );
}

export default DashboardHeader;
