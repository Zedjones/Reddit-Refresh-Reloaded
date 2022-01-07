import Drawer from '@material-ui/core/Drawer';
import List from '@material-ui/core/List';
import ListItem from '@material-ui/core/ListItem';
import ListSubheader from '@material-ui/core/ListSubheader';
import { ListItemText } from '@material-ui/core';
import { IconButton } from '@material-ui/core';
import { useGetUserSettingsQuery, useGetUserSearchesQuery, useDeleteSearchMutation, GetUserSearchesQuery } from '../types';
import { ListItemButton, ListItemIcon } from '@mui/material';
import SearchIcon from '@mui/icons-material/Search';
import RedditIcon from '@mui/icons-material/Reddit';
import AddIcon from '@mui/icons-material/AddCircleOutline';
import DeleteIcon from '@mui/icons-material/Delete';
import { useEffect, useMemo, useState } from 'react';
import * as _ from 'lodash';
import SearchDialog from './SearchDialog';
import SnackbarUtils from './SnackbarUtils';
import { useNavigate } from 'react-router-dom';

type SearchWithoutSub = Omit<GetUserSearchesQuery["getUserInfo"]["searches"][0], "subreddit">;

const NotifierItems = () => {
  const [result, refetch] = useGetUserSettingsQuery();

  const { data, fetching } = result;

  if (fetching) return <> </>;

  return (
    <div>
      {(data?.getUserInfo.settings.map(setting =>
        <ListItem button key={`setting-${setting.id}`}>
          <ListItemText primary={setting.name} />
        </ListItem>)) ?? []}
    </div>
  );
}

interface SidebarDrawerProps {
  drawerWidth: number;
}

export default function SidebarDrawer(props: SidebarDrawerProps) {
  const [result, refetch] = useGetUserSearchesQuery();
  const [deleteResult, deleteSearch] = useDeleteSearchMutation();
  const [searchDialogSub, setSearchDialogSub] = useState<string | undefined>();
  const [dialogOpen, setDialogOpen] = useState(false);
  const navigate = useNavigate();

  const { data, fetching } = result;


  const SearchItem = ([sub, search]: [string, SearchWithoutSub[]]) =>
    <>
      <ListItem key={sub} sx={{ pl: 4 }}>
        <ListItemIcon>
          <RedditIcon />
        </ListItemIcon>
        <ListItemText primary={sub} />
        <IconButton sx={{ alignContent: 'stretch' }} onClick={() => {
          setSearchDialogSub(sub);
          setOpen(true);
        }}>
          <AddIcon />
        </IconButton>
      </ListItem>
      <List component="div" key={`${sub}-list`} disablePadding>
        {search.map(search => (
          <ListItem key={search.searchTerm} sx={{ pl: 8 }} secondaryAction={
            <IconButton edge="end" onClick={() => {
              deleteSearch({
                id: search.id
              });
            }}>
              <DeleteIcon />
            </IconButton>
          }>
            <ListItemButton onClick={() => {
              navigate(`/searches/${search.id}`);
            }}>
              <ListItemIcon>
                <SearchIcon />
              </ListItemIcon>
              <ListItemText primary={search.searchTerm} />
            </ListItemButton>
          </ListItem>))}
      </List>
    </>;

  // Function to create an object mapping subreddit name to search
  const sortBySub = useMemo(() => {
    const searchArray = data?.getUserInfo.searches ?? [];
    const grouped = _.groupBy(searchArray, 'subreddit');
    const mapped = _.mapValues(grouped, search => search.map(key => _.omit(key, 'subreddit')));
    return mapped;
  }, [data]);

  const setOpen = (newValue: boolean) => {
    if (!newValue) {
      setSearchDialogSub(undefined);
    }
    setDialogOpen(newValue);
  };

  useEffect(() => {
    if (deleteResult.data) {
      SnackbarUtils.success('Successfully deleted search.');
    }
  }, [deleteResult]);

  return (
    <Drawer
      variant="persistent"
      sx={{
        display: { xs: 'none', sm: 'block' },
        '& .MuiDrawer-paper': { boxSizing: 'border-box', width: props.drawerWidth },
      }}
      open>
      <SearchDialog subreddit={searchDialogSub} open={dialogOpen} setOpen={setOpen} />
      <List>
        <ListSubheader>
          Searches
          <IconButton sx={{ alignContent: 'stretch' }} onClick={() => {
            setOpen(true);
          }}>
            <AddIcon />
          </IconButton>
        </ListSubheader>
        <List component="div" key='Subreddits' disablePadding>
          {Object.entries(sortBySub).map(SearchItem)}
        </List>
        <ListSubheader>
          Notifier Preferences
        </ListSubheader>
        <NotifierItems />
      </List >
    </Drawer >
  )
}