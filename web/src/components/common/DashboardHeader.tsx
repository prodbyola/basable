import * as React from 'react';
import { CssBaseline, List } from '@material-ui/core';
import ListItemText from '@mui/material/ListItemText';
import Button from '@mui/material/Button';
import ListItemAvatar from '@mui/material/ListItemAvatar';
import ToolBar from '@mui/material/Toolbar';
import AppBar from '@mui/material/AppBar';
import ListItem from '@mui/material/ListItem';
import AddCircleIcon from '@mui/icons-material/AddCircle';
import Typography from '@mui/material/Typography';
import { ListSubheader } from '@mui/material';
import { ThemeProvider } from "@mui/material/styles";
import theme from "../../theme";
import Logo from '../../assets/images/Basale-logo-white.svg';
import SearchBar from '../bar/SearchBar';
import useStyles from '../../styles/styles.js';
import Alarm from '../../assets/images/alarm.svg';
import Avatar from '../../assets/images/Avater.png';
import { Box } from '@mui/material';

const headerHeight = 80;

function DashboardHeader() {
  const classes = useStyles();
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
          height: headerHeight
        }}
      >
        <ToolBar className={classes.toolbar}>
          <img src={Logo} alt="Logo" style={{ height: 30, marginRight: 16 }} />
          <SearchBar />
          <div className={classes.headerright}>
            <Button
              variant="outlined"
              sx={{
                padding: '8px',
                borderRadius: '10px',
                marginRight: '50px',
                display: { sm: 'flex', xs: 'none' },
                width: 'auto'
              }}
            >
              <AddCircleIcon color="primary" />
              <Typography
                variant="subtitle2"
                sx={{
                  textTransform: 'none',
                  textWrap: 'nowrap',
                  marginLeft: '10px',
                  display: { md: 'block', sm: 'none' }
                }}
              >
                Connect new database
              </Typography>
            </Button>
            <Box
              sx={{
                display: { sm: 'flex', xs: 'none' }
              }}
            >
              <img src={Alarm} alt="alarm" />
              <List>
                <ListItem>
                  <ListItemAvatar>
                    <img
                      src={Avatar}
                      alt="avatar"
                      style={{ width: 30, height: 30 }}
                    />
                  </ListItemAvatar>

                  <ListItemText
                    primary="Stefania Asuqo"
                    secondary="stefaniaas@gmail.com"
                    sx={{
                      display: { sm: 'none', md: 'block' }
                    }}
                  ></ListItemText>
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
