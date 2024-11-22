import React, { useState } from 'react';
import { Typography, Box } from '@mui/material';
import backImg1 from '../../assets/images/signup_back1.svg';

const EmailVerification: React.FC = () => {
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        backgroundColor: 'white',
        position: 'relative',
        overflow: 'hidden'
      }}
    >
      <Box
        sx={{
          position: 'absolute',
          top: -150,
          left: -200
        }}
      >
        <img src={backImg1} width="500px" />
      </Box>
      <Box
        sx={{
          position: 'absolute',
          bottom: 430,
          right: 630
        }}
      >
        <img src={backImg1} />
      </Box>
      <Box
        sx={{
          position: 'absolute',
          top: 430,
          left: 630
        }}
      >
        <img src={backImg1} />
      </Box>
      <Box
        sx={{
          position: 'absolute',
          bottom: -200,
          right: -200
        }}
      >
        <img src={backImg1} width="500px" />
      </Box>

      <Box
        sx={{
          width: 400,
          padding: 4,
          borderRadius: 2,
          backgroundColor: '#fff',
          boxShadow: '0 4px 12px rgba(0, 0, 0, 0.1)',
          textAlign: 'center',
          position: 'relative'
        }}
      >
        <Typography
          variant="h4"
          component="h1"
          sx={{ fontWeight: 'bold', color: '#5a5fcf', mb: 1 }}
        >
          Email Verification
        </Typography>

        <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
          An email with a verification link was sent to <br />
          your email address a***@gmail.com
        </Typography>
      </Box>
    </Box>
  );
};

export default EmailVerification;
