import Avatar from '@mui/material/Avatar';
import Button from '@mui/material/Button';
import CssBaseline from '@mui/material/CssBaseline';
import TextField from '@mui/material/TextField';
import FormControlLabel from '@mui/material/FormControlLabel';
import Checkbox from '@mui/material/Checkbox';
import Link from '@mui/material/Link';
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';
import LockOutlinedIcon from '@mui/icons-material/LockOutlined';
import Typography from '@mui/material/Typography';
import Container from '@mui/material/Container';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import { useLoginMutation, useCreateUserMutation } from '../types';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useLocalStorage } from '@rehooks/local-storage';

const theme = createTheme();

interface SignInProps {
  signUp?: boolean;
}

export default function SignIn(props: SignInProps) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [refreshTime, setRefreshTime] = useState('5m');

  const [loginResult, submitLogin] = useLoginMutation();
  const [createResult, submitCreateUser] = useCreateUserMutation();
  const navigate = useNavigate();
  const [accessToken, setAccessToken] = useLocalStorage('accessToken');

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (props.signUp) {
      submitCreateUser({
        username,
        password,
        refreshTime: "5m",
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
              id="username"
              label="Username"
              name="username"
              autoComplete="username"
              autoFocus
              value={username}
              onChange={newVal => setUsername(newVal.target.value)}
            />
            <TextField
              margin="normal"
              required
              fullWidth
              name="password"
              label="Password"
              type="password"
              id="password"
              autoComplete="current-password"
              value={password}
              onChange={newVal => setPassword(newVal.target.value)}
            />
            <Button
              type="submit"
              fullWidth
              variant="contained"
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
