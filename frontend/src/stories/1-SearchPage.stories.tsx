import React from 'react';
import SearchPage from '../pages/SearchPage';

const config = {
    title: 'Search Page',
    component: SearchPage,
}

export const ToStorybook = () => <SearchPage />

ToStorybook.story = {
    name: 'Basic Search Page'
}

export default config;