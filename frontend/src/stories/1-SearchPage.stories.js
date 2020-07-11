import React from 'react';
import SearchPage from '../pages/SearchPage';

export default {
    title: 'Search Page',
    component: SearchPage,
}

export const ToStorybook = () => <SearchPage />

ToStorybook.story = {
    name: 'Basic Search Page'
}