import React, { useState } from 'react';

import DashboardHeader from '../components/common/DashboardHeader';
import DashboardNav from '../components/common/DashboardNav';
import DashboardMain from '../components/common/DashboardMain';
import Box from '@mui/material/Box';

function DashboardLayout() {
  const [ showSidebar, onShowSidebar ] = useState(false)
  return (
    <Box sx={{ display: 'flex ', flexWrap: {
      xs: 'wrap'
    } }}>
      <DashboardHeader onShowSidebar={ () => onShowSidebar(!showSidebar) } />
      <DashboardNav showMobileSidebar={showSidebar} />
      <DashboardMain />
    </Box>
  );
}

export default DashboardLayout;
