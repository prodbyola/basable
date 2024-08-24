import React from 'react';

import DashboardHeader from '../components/common/DashboardHeader';
import DashboardNav from '../components/common/DashboardNav';
import DashboardMain from '../components/common/DashboardMain';
import Box from '@mui/material/Box';

function DashboardLayout() {
  return (
    <Box sx={{ display: 'flex ' }}>
      <DashboardHeader />
      <DashboardNav />
      <DashboardMain />
    </Box>
  );
}

export default DashboardLayout;
