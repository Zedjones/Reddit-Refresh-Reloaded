import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';
import { createClient, subscriptionExchange, errorExchange, fetchExchange } from '@urql/core';
import { createClient as createWSClient } from 'graphql-ws';
import SnackbarUtils from './components/SnackbarUtils';

const getToken = () => {
  return localStorage.getItem('accessToken');
}

const wsClient = createWSClient({
  url: 'ws://localhost:8000/graphql'
});

export const client = createClient({
  // TODO: Change this depending on how we're running
  url: 'http://localhost:8000/graphql',
  fetchOptions: () => {
    const token = getToken();
    return {
      headers: {
        Authorization: token ? `Bearer ${token}` : ''
      }
    };
  },
  exchanges: [
    errorExchange({
      onError(error) {
        console.log(error);
        if (error.graphQLErrors.length > 0) {
          const firstError = error.graphQLErrors[0];
          // Go to the login page if we're not signed in or
          // the token is expired
          if (firstError.extensions.code === 401) {
            localStorage.removeItem('accessToken');
            window.location.assign('/login');
          }
          SnackbarUtils.error(firstError.message);
        }
        else if (error.networkError) {
          SnackbarUtils.error(error.networkError.message);
        }
      }
    }),
    fetchExchange,
    subscriptionExchange({
      forwardSubscription: (operation) => ({
        subscribe: (sink) => ({
          unsubscribe: wsClient.subscribe(operation, sink),
        }),
      }),
    }),
  ],
});

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();