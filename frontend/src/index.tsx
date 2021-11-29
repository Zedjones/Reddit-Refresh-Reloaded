import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import * as serviceWorker from './serviceWorker';
import { createClient, defaultExchanges, subscriptionExchange } from '@urql/core';
import { createClient as createWSClient } from 'graphql-ws';

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
    ...defaultExchanges,
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