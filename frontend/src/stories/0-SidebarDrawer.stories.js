import React from 'react';
import SidebarDrawer from '../components/SidebarDrawer';

export default {
    title: 'Sidebar Drawer',
    component: SidebarDrawer,
}

export const ToStorybook = () => <SidebarDrawer />

ToStorybook.story = {
    name: 'Basic Sidebar Drawer'
}