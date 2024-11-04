// web/src/components/forms/AuthForm.tsx
import React, { useState } from 'react';
import {
  Button,
  TextField,
  IconButton,
  InputAdornment,
  Typography,
  Link
} from '@mui/material';
import { Visibility, VisibilityOff } from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';

const AuthForm: React.FC = () => {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const navigate = useNavigate();

  const handleClickShowPassword = () => setShowPassword(!showPassword);
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log('Email:', email, 'Password:', password);

    navigate('/login');
  };

  return (
    <form onSubmit={handleSubmit}>
      <TextField
        label="Full Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
        variant="outlined"
        fullWidth
        sx={{
          mb: 2,
          '& .MuiOutlinedInput-root': {
            borderRadius: '10px' // makes the border circular
          }
        }}
      />

      <TextField
        label="Email"
        type="email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        fullWidth
        margin="normal"
        sx={{
          '& .MuiOutlinedInput-root': {
            borderRadius: '10px' // makes the border circular
          }
        }}
      />
      <TextField
        label="Password"
        type={showPassword ? 'text' : 'password'}
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        fullWidth
        margin="normal"
        InputProps={{
          endAdornment: (
            <InputAdornment position="end">
              <IconButton onClick={handleClickShowPassword} edge="end">
                {showPassword ? <VisibilityOff /> : <Visibility />}
              </IconButton>
            </InputAdornment>
          )
        }}
        sx={{
          '& .MuiOutlinedInput-root': {
            borderRadius: '10px' // makes the border circular
          }
        }}
      />
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
        Sign up
      </Button>
      <Typography variant="caption" color="textSecondary">
        By signing up, you agree to our{' '}
        <Link href="#">
          Terms and <br /> Service
        </Link>{' '}
        and <Link href="#">Privacy Policy</Link>.
      </Typography>

      <Typography variant="body2" color="black" sx={{ mt: 3 }}>
        Already have an account?{' '}
        <Link
          href="/login"
          color="primary"
          sx={{ color: '#00008b', textDecoration: 'none' }}
        >
          Login
        </Link>
      </Typography>
    </form>
  );
};

export default AuthForm;
