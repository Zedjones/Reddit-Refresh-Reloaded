import gql from 'graphql-tag';
import * as Urql from 'urql';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  DateTime: any;
  Duration: any;
};

/** Apprise Generic Configuration */
export type AppriseConfig = {
  __typename?: 'AppriseConfig';
  id: Scalars['Int'];
  /** Name of this configuration */
  name: Scalars['String'];
  /** Priority/urgency with which to send messages */
  urgency: Urgency;
  /** Apprise URI associated with this configuration */
  uri: Scalars['String'];
};

export enum ChangeType {
  Delete = 'DELETE',
  Insert = 'INSERT',
  Update = 'UPDATE'
}

export type Mutation = {
  __typename?: 'Mutation';
  addNotifier: AppriseConfig;
  addSearch: Search;
  /** Create a user with the provided username, password, and refresh time */
  createUser: Scalars['String'];
  deleteSearch: Scalars['Int'];
  login: Scalars['String'];
};


export type MutationAddNotifierArgs = {
  name: Scalars['String'];
  priority: Urgency;
  uri: Scalars['String'];
};


export type MutationAddSearchArgs = {
  refreshTime?: InputMaybe<Scalars['Duration']>;
  searchTerm: Scalars['String'];
  subreddit: Scalars['String'];
};


export type MutationCreateUserArgs = {
  password: Scalars['String'];
  refreshTime: Scalars['Duration'];
  username: Scalars['String'];
};


export type MutationDeleteSearchArgs = {
  id: Scalars['Int'];
};


export type MutationLoginArgs = {
  password: Scalars['String'];
  username: Scalars['String'];
};

export type Query = {
  __typename?: 'Query';
  getSearches: Array<Search>;
  getSearchesForSubreddit: Array<Search>;
  getUserInfo: User;
};


export type QueryGetSearchesForSubredditArgs = {
  subreddit: Scalars['String'];
};

export type Result = {
  __typename?: 'Result';
  id: Scalars['String'];
  inserted: Scalars['DateTime'];
  permalink: Scalars['String'];
  searchId: Scalars['Int'];
  title: Scalars['String'];
};

export type Search = {
  __typename?: 'Search';
  id: Scalars['Int'];
  refreshTime?: Maybe<Scalars['Duration']>;
  results: Array<Result>;
  searchTerm: Scalars['String'];
  subreddit: Scalars['String'];
  username: Scalars['String'];
};

export type SearchChange = {
  __typename?: 'SearchChange';
  operation: ChangeType;
  record: Search;
};

export type Subscription = {
  __typename?: 'Subscription';
  resultUpdates: Result;
  searchUpdates: SearchChange;
};


export type SubscriptionResultUpdatesArgs = {
  searchId: Scalars['Int'];
};

export enum Urgency {
  Failure = 'FAILURE',
  Info = 'INFO',
  Success = 'SUCCESS',
  Warning = 'WARNING'
}

export type User = {
  __typename?: 'User';
  refreshTime: Scalars['Duration'];
  searches: Array<Search>;
  settings: Array<AppriseConfig>;
  username: Scalars['String'];
};

export type GetUserSettingsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetUserSettingsQuery = { __typename?: 'Query', getUserInfo: { __typename?: 'User', settings: Array<{ __typename?: 'AppriseConfig', id: number, uri: string, name: string, urgency: Urgency }> } };

export type LoginMutationVariables = Exact<{
  username: Scalars['String'];
  password: Scalars['String'];
}>;


export type LoginMutation = { __typename?: 'Mutation', login: string };

export type CreateUserMutationVariables = Exact<{
  username: Scalars['String'];
  password: Scalars['String'];
  refreshTime: Scalars['Duration'];
}>;


export type CreateUserMutation = { __typename?: 'Mutation', createUser: string };


export const GetUserSettingsDocument = gql`
    query getUserSettings {
  getUserInfo {
    settings {
      id
      uri
      name
      urgency
    }
  }
}
    `;

export function useGetUserSettingsQuery(options: Omit<Urql.UseQueryArgs<GetUserSettingsQueryVariables>, 'query'> = {}) {
  return Urql.useQuery<GetUserSettingsQuery>({ query: GetUserSettingsDocument, ...options });
};
export const LoginDocument = gql`
    mutation login($username: String!, $password: String!) {
  login(username: $username, password: $password)
}
    `;

export function useLoginMutation() {
  return Urql.useMutation<LoginMutation, LoginMutationVariables>(LoginDocument);
};
export const CreateUserDocument = gql`
    mutation createUser($username: String!, $password: String!, $refreshTime: Duration!) {
  createUser(username: $username, password: $password, refreshTime: $refreshTime)
}
    `;

export function useCreateUserMutation() {
  return Urql.useMutation<CreateUserMutation, CreateUserMutationVariables>(CreateUserDocument);
};