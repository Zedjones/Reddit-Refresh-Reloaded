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
import { useFormik } from 'formik';
import * as yup from 'yup';
import { Backdrop } from '@mui/material';
import { CircularProgress } from '@mui/material';
// Have to use require since `duration-js` is not properly configured
// as an ES6 module
const Duration = require('duration-js');

const theme = createTheme();

interface SignInProps {
  signUp?: boolean;
}

interface LoginSignUpFormValues {
  username: string;
  password: string;
  refreshTime: string | undefined;
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
  const initialValues: LoginSignUpFormValues = { username: '', password: '', refreshTime: props.signUp ? '' : undefined };

  const refreshTimeValidation = () => {
    if (props.signUp) {
      return yup.string().optional().test('validate-duration', 'Must be in Rust/Go duration format, e.g. 1d15h5m4s, and be at least 5 seconds', value => {
        return value ? validateDuration(value) : true;
      });
    }
    else {
      return yup.string().optional();
    }
  }

  const passwordValidation = () => {
    let passwordYup = yup.string().required('A password is required');
    if (props.signUp) {
      passwordYup = passwordYup.min(8);
    }
    return passwordYup;
  }

  const [loginResult, submitLogin] = useLoginMutation();
  const [createResult, submitCreateUser] = useCreateUserMutation();
  const navigate = useNavigate();
  const [accessToken, setAccessToken] = useLocalStorage('accessToken');

  const formik = useFormik({
    initialValues,
    validationSchema: yup.object({
      username: yup.string().required('A username is required'),
      password: passwordValidation(),
      refreshTime: refreshTimeValidation(),
    }),
    onSubmit: values => {
      if (props.signUp) {
        submitCreateUser({
          ...values
        });
      } else {
        submitLogin({
          ...values
        });
      }
    }
  });

  useEffect(() => {
    if (accessToken) {
      navigate("/");
    }
  }, [accessToken, navigate]);

  const actionString = props.signUp ? 'Sign up' : 'Sign in';

  // Set access token when login returns
  useEffect(() => {
    if (loginResult.data) {
      setAccessToken(loginResult.data.login);
    }
  }, [loginResult.data, setAccessToken]);

  // Set access token when create user returns
  useEffect(() => {
    if (createResult.data) {
      setAccessToken(createResult.data.createUser);
    }
  }, [createResult.data, setAccessToken]);

  return (
    <ThemeProvider theme={theme}>
      <Backdrop
        sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1 }}
        open={createResult.fetching || loginResult.fetching}
      >
        <CircularProgress color="inherit" />
      </Backdrop>
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
          <Box component="form" onSubmit={formik.handleSubmit} noValidate sx={{ mt: 1 }}>
            <TextField
              margin="normal"
              required
              fullWidth
              id="username"
              label="Username"
              name="username"
              autoComplete="username"
              error={formik.touched.username && Boolean(formik.errors.username)}
              helperText={formik.touched.username && formik.errors.username}
              value={formik.values.username}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              autoFocus
            />
            <TextField
              margin="normal"
              required
              fullWidth
              name="password"
              label="Password"
              type="password"
              error={formik.touched.password && Boolean(formik.errors.password)}
              helperText={formik.touched.password && formik.errors.password}
              value={formik.values.password}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              id="password"
              autoComplete="current-password"
            />
            {props.signUp &&
              <TextField
                margin="normal"
                required
                fullWidth
                name="refreshTime"
                label="Refresh Time"
                type="text"
                id="refreshTime"
                value={formik.values.refreshTime}
                error={formik.touched.refreshTime && Boolean(formik.errors.refreshTime)}
                helperText={formik.touched.refreshTime && formik.errors.refreshTime}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
              />}
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
