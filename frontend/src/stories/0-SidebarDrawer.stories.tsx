import React from 'react';
import SidebarDrawer from '../components/SidebarDrawer';

const config = {
    title: 'Sidebar Drawer',
    component: SidebarDrawer,
}

export const ToStorybook = () => <SidebarDrawer drawerWidth={250} />

ToStorybook.story = {
    name: 'Basic Sidebar Drawer'
}

export default config;