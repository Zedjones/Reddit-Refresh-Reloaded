import Avatar from '@mui/material/Avatar';
import Button from '@mui/material/Button';
import CssBaseline from '@mui/material/CssBaseline';
import TextField from '@mui/material/TextField';
import Link from '@mui/material/Link';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';
import LockOutlinedIcon from '@mui/icons-material/LockOutlined';
import Typography from '@mui/material/Typography';
import Container from '@mui/material/Container';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { useLoginMutation, useCreateUserMutation } from '../types';
import { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useLocalStorage } from '@rehooks/local-storage';
import SnackbarUtils from './SnackbarUtils';
// Have to use require since `duration-js` is not properly configured
// as an ES6 module
const Duration = require('duration-js');

const theme = createTheme();

interface SignInProps {
  signUp?: boolean;
}

export const validateDuration = (durationStr: string) => {
  try {
    const duration = new Duration(durationStr);
    return duration.seconds() >= 5;
  }
  catch {
    return false;
  }
}

export default function SignIn(props: SignInProps) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [refreshTime, setRefreshTime] = useState('5m');
  const [refreshTimeError, setRefreshTimeError] = useState(false);

  const [loginResult, submitLogin] = useLoginMutation();
  const [createResult, submitCreateUser] = useCreateUserMutation();
  const navigate = useNavigate();
  const [accessToken, setAccessToken] = useLocalStorage('accessToken');

  const validateRefreshTime = useCallback(() => {
    return validateDuration(refreshTime);
  }, [refreshTime]);

  const usernameError = username === '';
  const passwordError = password.length < 8;
  const anyError = usernameError || passwordError || refreshTimeError;

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (anyError) {
      SnackbarUtils.error('Fix errors in form and resubmit.');
      return;
    }
    if (props.signUp) {
      submitCreateUser({
        username,
        password,
        refreshTime,
      })
    } else {
      submitLogin({
        username,
        password
      });
    }
  };



  useEffect(() => {
    if (accessToken) {
      navigate("/");
    }
  }, [accessToken, navigate]);

  useEffect(() => {
    setRefreshTimeError(!validateRefreshTime());
  }, [validateRefreshTime, setRefreshTimeError])

  const actionString = props.signUp ? 'Sign up' : 'Sign in';

  useEffect(() => {
    if (loginResult.data) {
      setAccessToken(loginResult.data.login);
    }
  }, [loginResult, setAccessToken]);

  useEffect(() => {
    if (createResult.data) {
      setAccessToken(createResult.data.createUser);
    }
  }, [createResult, setAccessToken]);

  return (
    <ThemeProvider theme={theme}>
      <Container component="main" maxWidth="xs">
        <CssBaseline />
        <Box
          sx={{
            marginTop: 8,
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
          }}
        >
          <Avatar sx={{ m: 1, bgcolor: 'secondary.main' }}>
            <LockOutlinedIcon />
          </Avatar>
          <Typography component="h1" variant="h5">
            {actionString}
          </Typography>
          <Box component="form" onSubmit={handleSubmit} noValidate sx={{ mt: 1 }}>
            <TextField
              margin="normal"
              required
              fullWidth
              error={usernameError}
              id="username"
              label="Username"
              name="username"
              autoComplete="username"
              helperText={usernameError ? 'Username must not be empty' : ''}
              autoFocus
              value={username}
              onChange={newVal => setUsername(newVal.target.value)}
            />
            <TextField
              margin="normal"
              required
              fullWidth
              error={passwordError}
              name="password"
              label="Password"
              type="password"
              helperText={passwordError ? 'Password must be at least 8 characters' : ''}
              id="password"
              autoComplete="current-password"
              value={password}
              onChange={newVal => setPassword(newVal.target.value)}
            />
            {props.signUp &&
              <TextField
                margin="normal"
                error={refreshTimeError}
                required
                fullWidth
                helperText={refreshTimeError ? 'Must be in Rust/Go duration format, e.g. 1d15h5m4s, and be at least 5 seconds' : ''}
                name="refresh-time"
                label="Refresh Time"
                type="text"
                id="refresh-time"
                value={refreshTime}
                onChange={newVal => setRefreshTime(newVal.target.value)}
              />}
            <Button
              type="submit"
              fullWidth
              variant="contained"
              disabled={anyError}
              sx={{ mt: 3, mb: 2 }}
            >
              {actionString}
            </Button>
            {!props.signUp &&
              <Grid container>
                <Grid item>
                  <Link href="/signup" variant="body2">
                    {"Don't have an account? Sign Up"}
                  </Link>
                </Grid>
              </Grid>
            }
          </Box>
        </Box>
      </Container>
    </ThemeProvider>
  );
}
