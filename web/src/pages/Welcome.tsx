// web/src/components/forms/AuthForm.tsx
import React, { useState } from 'react';
import { Box, Button, TextField, Typography, Link } from '@mui/material';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import backImg1 from '../assets/images/signup_back1.svg';

interface ForgotPasswordFormProps {
  showEmailVerification: React.Dispatch<React.SetStateAction<boolean>>;
}

const roles = [
  'Full-Stack Developer',
  'Front-End Developer',
  'Back-End Developer',
  'Software Engineer',
  'Blockchain Developer',
  'Database Engineer',
  'Machine Learning Engineer',
  'Data Scientist'
];

const Welcome = () => {
  const [company, setCompany] = useState('');
  const [role, setRole] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
  };

  const handleChange = (event: SelectChangeEvent) => {
    setRole(event.target.value as string);
  };

  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100vh',
        backgroundColor: 'white'
      }}
    >
      <Box
        sx={{
          position: 'absolute',
          bottom: 500,
          right: 550
        }}
      >
        <img src={backImg1} width="400px" />
      </Box>
      <Box
        sx={{
          position: 'absolute',
          top: 500,
          left: 550
        }}
      >
        <img src={backImg1} width="400px" />
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
          Welcome to Basable
        </Typography>

        <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
          Let's get started with your data journey.
        </Typography>

        <form onSubmit={handleSubmit}>
          <TextField
            placeholder="Business/Company role"
            variant="outlined"
            value={company}
            onChange={(e) => setCompany(e.target.value)}
            fullWidth
            sx={{
              mb: 2,
              '& .MuiOutlinedInput-root': {
                borderRadius: '10px' // makes the border circular
              }
            }}
          />

          <FormControl
            fullWidth
            sx={{
              mb: 2,
              '& .MuiOutlinedInput-root': {
                borderRadius: '10px' // makes the border circular
              }
            }}
          >
            <InputLabel id="demo-multiple-role-label">Role</InputLabel>
            <Select
              labelId="demo-simple-select-label"
              id="demo-simple-select"
              value={role}
              label="role"
              onChange={handleChange}
            >
              {roles.map((role) => (
                <MenuItem key={role} value={role}>
                  {role}
                </MenuItem>
              ))}
            </Select>
          </FormControl>

          <Button
            type="submit"
            variant="contained"
            fullWidth
            sx={{
              backgroundColor: '#5a5fcf',
              color: '#fff',
              textTransform: 'none',
              my: 2
            }}
          >
            Proceed
          </Button>
        </form>
      </Box>
    </Box>
  );
};

export default Welcome;
