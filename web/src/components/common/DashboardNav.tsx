import * as React from 'react';
import { List, ListItemText } from '@material-ui/core';
import ToolBar from '@mui/material/Toolbar';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import Divider from '@mui/material/Divider';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import DashboardIcon from '@mui/icons-material/DashboardOutlined';
import AnalyticsIcon from '@mui/icons-material/AnalyticsOutlined';
import TableChartIcon from '@mui/icons-material/TableChartOutlined';
import FiberManualRecordIcon from '@mui/icons-material/FiberManualRecord';
import HelpIcon from '@mui/icons-material/HelpCenterOutlined';
import LogOutIcon from '@mui/icons-material/LogoutOutlined';
import MenuItem from '@mui/material/MenuItem';
import Collapse from '@mui/material/Collapse';
import { ListSubheader } from '@mui/material';

import useStyles from '../../styles/styles.js';
import Avatar from '../../assets/images/Avater.png';
import database1 from '../../assets/images/database1.svg';
import database2 from '../../assets/images/database2.svg';
import database3 from '../../assets/images/database3.svg';
import database4 from '../../assets/images/database4.svg';

import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';

import ExpandLess from '@mui/icons-material/ExpandLess';
import ExpandMore from '@mui/icons-material/ExpandMore';

const drawerWidth = 240;

interface Item {
  id: number;
  title: string;
}
const tableitems: Item[] = [
  { id: 1, title: 'Table 1' },
  { id: 2, title: 'Table 2' },
  { id: 3, title: 'Table 3' },
  { id: 4, title: 'Table 4' },
  { id: 5, title: 'Table 5' }
];

function DashboardNav() {
  const classes = useStyles();

  const [openTables, setOpenTables] = React.useState(false);
  const [openDatabase, setOpenDatabase] = React.useState(false);

  const handleTablesClick = () => {
    setOpenTables(!openTables);
  };

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: drawerWidth,
        flexShrink: 0,
        [`& .MuiDrawer-paper`]: {
          width: drawerWidth,
          boxSizing: 'border-box'
        }
      }}
    >
      <ToolBar sx={{ height: '100px' }}></ToolBar>
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'space-between',
          overflow: 'auto',
          padding: '20px',
          height: 'calc(100% - 100px)'
        }}
      >
        <div className="main-container">
          <ListItem>
            <div style={{ paddingRight: '10px' }}>
              <img src={Avatar} alt="avatar" />
            </div>
            <ListItemText
              primary={<strong>Stefania Asuqo</strong>}
              secondary={
                <div
                  style={{
                    display: 'inline-flex',
                    paddingLeft: '5px',
                    paddingRight: '5px',
                    backgroundColor: 'red',
                    borderRadius: '5px',
                    color: 'white'
                  }}
                >
                  Developer
                </div>
              }
            ></ListItemText>
          </ListItem>
          <ListItem sx={{ paddingLeft: '0px' }}>
            <Box>
              <FormControl fullWidth>
                <Select
                  defaultValue={10}
                  sx={{
                    '& .MuiOutlinedInput-notchedOutline': {
                      border: 'none' // Remove the border for outlined variant
                    },
                    '& .MuiInputBase-input': {
                      // Add specific styles for InputBase here
                      display: 'flex'
                    }
                  }}
                >
                  <ListSubheader>
                    <strong>Connected Database</strong>
                  </ListSubheader>
                  <Box sx={{ paddingX: '15px' }}>
                    <Divider />
                  </Box>
                  <MenuItem value={10}>
                    <img
                      src={database1}
                      alt=""
                      style={{ paddingRight: '5px' }}
                    />
                    Basable Database
                  </MenuItem>
                  <MenuItem value={20}>
                    <img
                      src={database2}
                      alt=""
                      style={{ paddingRight: '5px' }}
                    />
                    Jumia Database
                  </MenuItem>
                  <MenuItem value={30}>
                    <img
                      src={database3}
                      alt=""
                      style={{ paddingRight: '5px' }}
                    />
                    Whatsapp Database
                  </MenuItem>
                  <MenuItem value={40}>
                    <img
                      src={database4}
                      alt=""
                      style={{ paddingRight: '5px' }}
                    />
                    Instagram Database
                  </MenuItem>
                </Select>
              </FormControl>
            </Box>
          </ListItem>
          <Divider />
          <ListItemButton
            sx={{
              mt: '10px',
              '&.Mui-selected': {
                border: `1px solid`,
                borderColor: 'blue',
                borderRadius: '10px'
              }
            }}
            selected
          >
            <ListItemIcon>
              <DashboardIcon color="primary" />
            </ListItemIcon>
            <ListItemText primary="Dashboard" />
          </ListItemButton>
          <ListItemButton onClick={handleTablesClick}>
            <ListItemIcon>
              <TableChartIcon />
            </ListItemIcon>
            <ListItemText primary="Tables" />
            {openTables ? <ExpandLess /> : <ExpandMore />}
          </ListItemButton>
          <Collapse in={openTables} timeout="auto" unmountOnExit>
            <List component="div">
              {tableitems.map((item) => (
                <ListItemButton key={item.id} sx={{ pl: 4, pt: 0, pb: 0 }}>
                  <ListItemIcon>
                    <FiberManualRecordIcon fontSize="small" />
                  </ListItemIcon>
                  <ListItemText primary={item.title} />
                </ListItemButton>
              ))}
            </List>
          </Collapse>
          <ListItemButton>
            <ListItemIcon>
              <AnalyticsIcon />
            </ListItemIcon>
            <ListItemText primary="Analysics" />
          </ListItemButton>
        </div>
        <div className="bottom-container">
          <ListItemButton>
            <ListItemIcon>
              <HelpIcon />
            </ListItemIcon>
            <ListItemText primary="Help" />
          </ListItemButton>
          <ListItemButton>
            <ListItemIcon>
              <LogOutIcon />
            </ListItemIcon>
            <ListItemText primary="LogOut" />
          </ListItemButton>
        </div>
      </Box>
    </Drawer>
  );
}

export default DashboardNav;
