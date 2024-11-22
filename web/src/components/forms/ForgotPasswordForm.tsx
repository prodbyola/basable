// web/src/components/forms/AuthForm.tsx
import React, { useState } from 'react';
import { Box, Button, TextField, Typography, Link } from '@mui/material';
import backImg1 from '../../assets/images/signup_back1.svg';

interface ForgotPasswordFormProps {
  showEmailVerification: React.Dispatch<React.SetStateAction<boolean>>;
}

const ForgotPasswordForm: React.FC<ForgotPasswordFormProps> = ({
  showEmailVerification
}) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    showEmailVerification(true);
    e.preventDefault();
    console.log('Email:', email, 'Password:', password);
  };

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
          Reset Password
        </Typography>

        <Typography variant="body2" color="textSecondary" sx={{ mb: 3 }}>
          Enter your email address below to receive a <br />
          password reset link.
        </Typography>

        <form onSubmit={handleSubmit}>
          <TextField
            placeholder="Your Email Address"
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
            Send Reset Link
          </Button>

          <Typography variant="body2" color="black" sx={{ mt: 3 }}>
            Having trouble?{' '}
            <Link
              href="#"
              color="primary"
              sx={{ color: '#00008b', textDecoration: 'none' }}
            >
              Contact support
            </Link>
          </Typography>
        </form>
      </Box>
    </Box>
  );
};

export default ForgotPasswordForm;
