import React, { useState } from 'react';
import {
  Button,
  TextField,
  IconButton,
  InputAdornment,
  Typography,
  Link,
  Box
} from '@mui/material';
import ForgotPasswordForm from '../components/forms/ForgotPasswordForm';
import EmailVerification from '../components/forms/EmailVerification';

const AuthPage: React.FC = () => {
  const [showVerifyPage, setShowVerifyPage] = useState(false);

  return (
    <Box>
      {showVerifyPage ? (
        <EmailVerification />
      ) : (
        <ForgotPasswordForm showEmailVerification={setShowVerifyPage} />
      )}
    </Box>
  );
};

export default AuthPage;
